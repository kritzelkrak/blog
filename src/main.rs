use comrak::{markdown_to_html, Options};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String
}

/* Parses a post into its frontmatter and content. */
fn parse(raw: &str) -> (FrontMatter, String) {
    let mut parts = raw.splitn(3, "+++");        // ["", "\ntitle", "\ncontent"]
    parts.next();                                // skip first ""

    let fm_raw = parts.next().expect("Missing frontmatter!");
    let content = parts.next().expect("Missing content!");

    let fm: FrontMatter = toml::from_str(fm_raw)
        .expect("Invalid frontmatter!");

    (fm, content.to_string())
}

fn main() {
    let template = fs::read_to_string("templates/post.html").unwrap();

    fs::create_dir_all("build").unwrap();

    // read all the posts
    for file in fs::read_dir("site").unwrap() {
        let path = file.unwrap().path();
        let post = fs::read_to_string(&path).unwrap();

        let (fm, content) = parse(&post);
        let html = markdown_to_html(&content, &Options::default());
        let page = template
            .replace("{{title}}", &fm.title)
            .replace("{{content}}", &html);

        let new_path = format!("build/{}.html", path.file_stem().unwrap().to_string_lossy());
        fs::write(new_path, page).unwrap();
   }
}
