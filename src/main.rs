pub mod markdown;

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::Router;
use clap::{arg, Command};
use include_dir::{include_dir, Dir};
use markdown::{html_post_proces, render_md};
use minijinja::{context, path_loader};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/examples/");

#[derive(Deserialize, Serialize, Debug, Clone)]
struct NavItem {
    title: String,
    href: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct VaultConfig {
    pub source: PathBuf,
    pub dist: PathBuf,
    pub templates: PathBuf,
    pub statics: PathBuf,
    pub base_url: String,
    pub(crate) nav: Vec<NavItem>,
}

#[derive(Clone)]
struct Site {
    config: VaultConfig,
    templates: minijinja::Environment<'static>,
}

impl Site {
    fn new(base_url: Option<&String>) -> Self {
        let config_file = fs::File::open("vault.json").expect("Unable to open the file");
        let mut config: VaultConfig = serde_json::from_reader(config_file).unwrap();
        if let Some(base_url) = base_url {
            config.base_url = base_url.clone();
        }
        if config.base_url.ends_with("/") {
            config.base_url.pop();
        }

        let mut templates = minijinja::Environment::new();
        templates.set_loader(path_loader(&config.templates));
        Self { config, templates }
    }

    fn render_md(&self, contents: &str, path: &std::path::Path) -> String {
        let html_output = render_md(contents);
        let html_output = html_post_proces(&html_output, path);

        let template = self.templates.get_template("page.html").unwrap();
        let html_output: String = template
            .render(context! {
                base_url => self.config.base_url,
                title => path.file_stem().unwrap().to_str().unwrap(),
                note_html => html_output,
                nav => self.config.nav,
            })
            .unwrap();

        html_output
    }

    fn dfs(&self, path: &std::path::Path, output_path: &std::path::Path) {
        if path.is_dir() {
            let paths = fs::read_dir(path).expect("Directory not exists");
            for path2 in paths {
                let path2 = path2.unwrap().path();
                let output_path2 = output_path.join(path2.file_name().unwrap());
                self.dfs(&path2, &output_path2);
            }
        } else if path.extension().unwrap_or_default() == "md" {
            println!("{}", path.display());
            let contents = fs::read_to_string(path).expect("Unable to read file");
            let html_output = self.render_md(&contents, path);
            let output_path = output_path.with_extension("html");
            create_dir_if_not_exists(&output_path);

            fs::write(output_path, html_output).expect("Unable to write the file");
        } else {
            create_dir_if_not_exists(output_path);
            fs::copy(path, output_path).expect("Unable to copy file");
        }
    }
    fn build(&self) {
        self.dfs(&self.config.source, &self.config.dist);
        self.dfs(&self.config.statics, &self.config.dist.join("_static"));
    }
    async fn serve(&self, host: &str) {
        let state = Arc::new(self.clone());
        let app = Router::new()
            .route("/{*path}", get(server_render))
            .route(
                "/",
                get(|| async { Redirect::permanent("/index.html").into_response() }),
            )
            .with_state(state);

        println!("Listening on http://{}{}", host, self.config.base_url);
        let listener = TcpListener::bind(host).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

async fn server_render(Path(url): Path<String>, State(site): State<Arc<Site>>) -> Response {
    let url = {
        let base_url = site.config.base_url.trim_start_matches('/');
        let url = url.strip_prefix(base_url).unwrap_or(&url);
        url.trim_start_matches('/')
    };
    let path = PathBuf::from(&url);
    let mut path = if path.starts_with("_static/") {
        site.config
            .statics
            .join(path.strip_prefix("_static/").unwrap())
    } else {
        site.config.source.join(&path)
    };
    if path.is_dir() {
        path = path.join("index.html");
    }

    if path.extension().unwrap_or_default() == "md" {
        let suf = url.strip_suffix(".md").unwrap();
        let redirect_url = format!("{suf}.html");
        return Redirect::permanent(&redirect_url).into_response();
    }
    if path.exists() {
        let contents = fs::read_to_string(path).expect("Unable to read file");
        return Response::new(contents.into());
    }

    path.set_extension("md");
    if path.exists() {
        let contents = fs::read_to_string(&path).expect("Unable to read file");
        let html_output = site.render_md(&contents, &path);
        return Response::new(html_output.into());
    }
    Response::new("Not found".into())
}

fn create_dir_if_not_exists(path: &std::path::Path) {
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
        .subcommand(
            Command::new("serve")
                .about("Serve the markdown files")
                .arg(arg!(<path> "The directory of the markdown files"))
                .arg(arg!(-H --"host" ["host"] "The host to listen on"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new vault")
                .arg(arg!(<path> "The directory to initialize the vault in"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("update-templates")
                .about("Update the templates")
                .arg(arg!(<path> "The directory to update the templates in"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            site.build();
        }
        Some(("serve", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .expect("required")
                .to_string();
            let host = if let Some(host) = sub_matches.get_one::<String>("host") {
                host.to_string()
            } else {
                "127.0.0.1:3000".to_string()
            };
            std::env::set_current_dir(&path)?;

            let site = Site::new(None);
            site.serve(&host).await;
        }
        Some(("init", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .expect("required")
                .to_string();
            PROJECT_DIR.extract(&path).unwrap();
        }
        Some(("update-templates", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .expect("required")
                .to_string();
            let templates_path = PathBuf::from(&path).join("templates");
            if templates_path.exists() {
                fs::remove_dir_all(&templates_path).unwrap();
            }
            PROJECT_DIR
                .get_dir("templates")
                .unwrap()
                .extract(path)
                .unwrap();
        }
        _ => {
            eprintln!("Please provide a subcommand");
            std::process::exit(0);
        }
    }

    Ok(())
}
