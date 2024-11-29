use minijinja::{context, path_loader};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::fs::{self};
use std::path::{Path, PathBuf};

fn render_md(md: &str, path: &Path) -> String {
    let parser = Parser::new_ext(md, Options::all());

    let parser = parser.map(|event| match event {
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            if let Ok(url) = resolve_links(&dest_url, path) {
                Event::Start(Tag::Link {
                    link_type,
                    dest_url: url.into(),
                    title,
                    id,
                })
            } else {
                eprintln!("Failed to resolve link: {}", dest_url);
                Event::Start(Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                })
            }
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    // html_output
    html_output
}

fn split_query(input: &str) -> (&str, Option<&str>) {
    if let Some(index) = input.find(['?', '#'].as_ref()) {
        let (head, tail) = input.split_at(index);
        (head, Some(&tail))
    } else {
        (input, None)
    }
}

fn resolve_links(url: &str, md_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    if url.contains("://") {
        return Ok(url.to_owned());
    }
    let (path, query) = split_query(url);
    let new_path = md_path.parent().unwrap().join(path);
    if !new_path.with_extension("md").exists() {
        return Err(format!("File not found: {}", new_path.display()).into());
    }
    let mut new_path = String::new();
    if path.ends_with(".md") {
        new_path.push_str(&path[..path.len() - 3]);
        new_path.push_str(".html");
    } else {
        new_path.push_str(path);
        new_path.push_str(".html");
    }
    if let Some(query) = query {
        new_path.push_str(query);
    }
    return Ok(new_path);
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
                title => "Test",
                note_html => html_output,
            })
            .unwrap();

        let relative_path = path.strip_prefix(&vault.source).unwrap();
        let output_path = vault.dist.join(relative_path).with_extension("html");
        fs::write(output_path, html_output).expect("Unable to write the file");
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
