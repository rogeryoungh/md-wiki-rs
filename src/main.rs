use lol_html::html_content::Element;
use lol_html::{element, HtmlRewriter, Settings};
use minijinja::{context, path_loader};
use pulldown_cmark::{html, Options, Parser};
use std::fs::{self};
use std::path::{Path, PathBuf};

fn render_md(md: &str, path: &Path) -> String {
    let parser = Parser::new_ext(md, Options::all());

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    // html_output
    html_post_proces(&html_output, path)
}

fn html_post_proces(html: &str, md_path: &Path) -> String {
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

fn split_query(input: &str) -> (&str, Option<&str>) {
    if let Some(index) = input.find(['?', '#'].as_ref()) {
        let (head, tail) = input.split_at(index);
        (head, Some(tail))
    } else {
        (input, None)
    }
}

fn resolve_links(el: &mut Element, md_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

fn dfs(path: &PathBuf, vault: &Vault) {
    if path.is_dir() {
        let paths = fs::read_dir(path).expect("Directory not exists");
        for path in paths {
            let path = path.unwrap().path();
            dfs(&path, vault);
        }
    } else if path.extension().unwrap_or_default() == "md" {
        println!("{}", path.display());
        let contents = fs::read_to_string(path).expect("Unable to read file");

        let html_output = render_md(&contents, path);

        let mut env = minijinja::Environment::new();
        env.set_loader(path_loader(&vault.templates));
        let template = env.get_template("page.html").unwrap();
        let html_output: String = template
            .render(context! {
                title => path.file_stem().unwrap().to_str().unwrap(),
                note_html => html_output,
            })
            .unwrap();

        let relative_path = path.strip_prefix(&vault.source).unwrap();
        let output_path = vault.dist.join(relative_path).with_extension("html");

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Unable to create directory");
        }

        fs::write(output_path, html_output).expect("Unable to write the file");
    } else {
        let relative_path = path.strip_prefix(&vault.source).unwrap();
        let output_path = vault.dist.join(relative_path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Unable to create directory");
        }
        fs::copy(path, output_path).expect("Unable to copy file");
    }
}

struct Vault {
    source: PathBuf,
    dist: PathBuf,
    templates: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault = Vault {
        source: PathBuf::from("./examples/docs"),
        dist: PathBuf::from("./examples/dist"),
        templates: PathBuf::from("./examples/templates"),
    };

    // 创建 dist 文件夹
    if vault.dist.exists() {
        fs::remove_dir_all(&vault.dist)?;
    }
    fs::create_dir(&vault.dist)?;

    // 遍历所有 Markdown 文件
    dfs(&vault.source, &vault);

    Ok(())
}
