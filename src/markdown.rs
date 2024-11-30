use lol_html::html_content::Element;
use lol_html::{element, HtmlRewriter, Settings};
use pulldown_cmark::{html, Options, Parser};
use std::path::Path;

pub fn render_md(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn html_post_proces(html: &str, md_path: &Path) -> String {
    let mut output = Vec::new();
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("a[href]", |el| {
                resolve_links(el, md_path).unwrap();
                Ok(())
            })],
            ..Settings::new()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );
    rewriter.write(html.as_bytes()).unwrap();

    String::from_utf8(output).unwrap()
}

pub fn split_query(input: &str) -> (&str, Option<&str>) {
    if let Some(index) = input.find(['?', '#'].as_ref()) {
        let (head, tail) = input.split_at(index);
        (head, Some(tail))
    } else {
        (input, None)
    }
}

pub fn resolve_links(el: &mut Element, md_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let url = el.get_attribute("href").expect("href was required");

    if url.contains("://") || url.starts_with('/') {
        el.set_attribute("target", "_blank").unwrap();
        return Ok(());
    }
    let url = urlencoding::decode(&url).unwrap().into_owned();
    let (path, query) = split_query(&url);
    let path = path.strip_prefix("./").unwrap_or(path);
    let new_path = md_path.parent().unwrap().join(path);
    if !new_path.with_extension("md").exists() {
        return Err(format!("File not found: {}", new_path.display()).into());
    }
    let mut new_path = String::new();
    new_path.push_str(path.strip_suffix(".md").unwrap_or(path));
    new_path.push_str(".html");
    if let Some(query) = query {
        new_path.push_str(query);
    }

    el.set_attribute("href", &new_path).unwrap();
    Ok(())
}
