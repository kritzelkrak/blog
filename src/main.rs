use comrak::{markdown_to_html, Options};
use std::fs;

fn main() {
    let template = fs::read_to_string("templates/post.html").unwrap();
    let post = fs::read_to_string("site/first.md").unwrap();

    let content = markdown_to_html(&post, &Options::default());
    let page = template
        .replace("{{title}}", "temp hardcoded title")
        .replace("{{content}}", &content);

    fs::create_dir_all("build").unwrap();
    fs::write("build/index.html", page).unwrap();
}
