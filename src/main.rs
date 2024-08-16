use askama::Template;
use chrono::prelude::*;
use std::fs::*;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use structs::HomeTemplate;

mod structs;

fn render_to_file(content: String, path: &String) -> io::Result<()> {
    let mut content_path = PathBuf::from("./static/");
    content_path.push(path);

    println!("{content_path:?}");

    let mut file = File::create(content_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let articles = [structs::Article {
        title: String::from("The life of times of qud"),
        subtitle: String::from("lmfao"),
        author: String::from("Joesphus"),
        content_path: String::from("assets/roman_satire/roman_satire.md"),
        link: String::from("articles/roman_satire.html"),
        date: Local::now().date_naive(),
    }];
    let homepage = HomeTemplate {
        articles: &articles,
    };

    render_to_file(homepage.render().unwrap(), &String::from("index.html"))?;
    println!("rendered main");
    for article in articles.iter() {
        println!("trying to render articles");
        render_to_file(article.create_template()?.render().unwrap(), &article.link)?;
    }
    Ok(())
}
