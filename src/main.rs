use askama::Template;
use chrono::prelude::*;
mod structs;

fn render_to_file(file: &String) {}

fn main() {
    let test_article = structs::EditorialTemplate {
        title: &String::from("The life of times of qud"),
        author: &String::from("Joesphus"),
        content: &String::from("../static/articles/article.md"),
        date: &Local::now().date_naive().to_string(),
    };

    let rendered = test_article.render().unwrap();

    println!("{rendered}");
}
