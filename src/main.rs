pub mod markdown;

use clap::{arg, Command};
use markdown::{html_post_proces, render_md};
use minijinja::{context, path_loader};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct VaultConfig {
    pub source: PathBuf,
    pub dist: PathBuf,
    pub templates: PathBuf,
    pub base_url: String,
}

struct Site<'a> {
    config: VaultConfig,
    templates: minijinja::Environment<'a>,
}

impl Site<'_> {
    fn new(base_url: Option<&String>) -> Self {
        let config_file = fs::File::open("vault.json").expect("Unable to open the file");
        let mut config: VaultConfig = serde_json::from_reader(config_file).unwrap();
        if let Some(base_url) = base_url {
            config.base_url = base_url.to_string();
        }

        let mut templates = minijinja::Environment::new();
        templates.set_loader(path_loader(&config.templates));
        Self { config, templates }
    }

    fn dfs(&self, path: &Path) {
        if path.is_dir() {
            let paths = fs::read_dir(path).expect("Directory not exists");
            for path in paths {
                let path = path.unwrap().path();
                self.dfs(&path);
            }
        } else if path.extension().unwrap_or_default() == "md" {
            println!("{}", path.display());
            let contents = fs::read_to_string(path).expect("Unable to read file");

            let html_output = render_md(&contents);
            let html_output = html_post_proces(&html_output, path);

            let template = self.templates.get_template("page.html").unwrap();
            let html_output: String = template
                .render(context! {
                    base_url => self.config.base_url,
                    title => path.file_stem().unwrap().to_str().unwrap(),
                    note_html => html_output,
                })
                .unwrap();

            let relative_path = path.strip_prefix(&self.config.source).unwrap();
            let output_path = self.config.dist.join(relative_path).with_extension("html");
            create_dir_if_not_exists(&output_path);

            fs::write(output_path, html_output).expect("Unable to write the file");
        } else {
            let relative_path = path.strip_prefix(&self.config.source).unwrap();
            let output_path = self.config.dist.join(relative_path);
            create_dir_if_not_exists(&output_path);
            fs::copy(path, output_path).expect("Unable to copy file");
        }
    }
}

fn create_dir_if_not_exists(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Unable to create directory");
    }
}

fn cli() -> Command {
    Command::new("md-wiki-rs")
        .about("A simple markdown generator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("build")
                .about("Build the markdown files")
                .arg(arg!(<path> "The directory of the markdown files"))
                .arg(arg!(-b --"base-url" ["base-url"] "Override the base URL"))
                .arg_required_else_help(true),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .expect("required")
                .to_string();
            let base_url = sub_matches.get_one::<String>("base-url");
            std::env::set_current_dir(&path)?;

            let site = Site::new(base_url);
            site.dfs(&site.config.source);
        }
        _ => {
            eprintln!("Please provide a subcommand");
            std::process::exit(0);
        }
    }

    Ok(())
}
