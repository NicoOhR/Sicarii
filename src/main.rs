use askama::Template;
use chrono::prelude::*;
use std::fs::*;
use std::io;
use std::io::Write;

mod structs;

fn render_to_file(content: &String, path: &String) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() {
    let test_article = structs::EditorialTemplate {
        title: String::from("The life of times of qud"),
        author: String::from("Joesphus"),
        content: String::from("../static/articles/article.md"),
        date: Local::now().date_naive().to_string(),
    };

    let rendered = test_article.render().unwrap();

    println!("{rendered}");
}
