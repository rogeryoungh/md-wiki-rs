use std::fs;

use minijinja::{context, path_loader};

fn main() { // Create parser with example Markdown text.
    let markdown_input = fs::read_to_string("./examples/docs/blog.md").unwrap();

    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, pulldown_cmark::Options::all());
    
    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let mut env = minijinja::Environment::new();
    env.set_loader(path_loader("./examples/templates/"));
    let template = env.get_template("page.html").unwrap();
    let output = template.render(context! {
        title => "Test",
        note_html => html_output,
    }).unwrap();
    fs::write("./examples/dist/test.html", output).unwrap();
    
}
