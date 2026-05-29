use crate::article;
use std::fs;
use std::io;
use walkdir::WalkDir;

fn read_toml(s: &str) -> article::Article {
    let article: article::Article = toml::from_str(s).unwrap();
    article
}

pub fn get_articles() -> io::Result<Vec<article::Article>> {
    let dir_path = "./assets/";
    let mut articles: Vec<article::Article> = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "toml" {
                    articles.push(read_toml(&fs::read_to_string(path)?));
                }
            }
        }
    }

    articles.sort_by(|a, b| b.cmp(a));

    Ok(articles)
}
