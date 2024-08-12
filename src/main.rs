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

fn main() -> io::Result<()> {
    let test_article = structs::Article {
        title: String::from("The life of times of qud"),
        subtitle: String::from("lmfao"),
        author: String::from("Joesphus"),
        path: String::from("static/articles/article.md"),
        date: Local::now().date_naive(),
    };

    render_to_file(
        &test_article.create_template()?.render().unwrap(),
        &String::from("static/articles/test_article/test.html"),
    )?;
    Ok(())
}
