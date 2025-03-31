use askama::Template;
use std::fs::*;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use structs::HomeTemplate;

mod article_meta;
mod structs;

fn render_to_file(content: String, path: &str) -> io::Result<()> {
    println!("Created Content Path");
    let mut content_path = PathBuf::from("./site/");
    content_path.push(path);
    println!("{content_path:?}");

    if let Some(parent) = content_path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = File::create(content_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let articles = article_meta::get_articles()?;

    let homepage = HomeTemplate {
        articles: &articles,
    };

    render_to_file(homepage.render().unwrap(), &String::from("index.html"))?;
    println!("rendered main");
    for article in articles.iter() {
        println!("trying to render articles");
        println!("{}", &article.link);
        render_to_file(article.create_template()?.render().unwrap(), &article.link)?;
    }
    Ok(())
}
