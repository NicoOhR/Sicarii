use crate::structs;
use std::fs;
use std::io;
use walkdir::WalkDir;

fn read_toml(s: &str) -> structs::Article {
    let article: structs::Article = toml::from_str(s).unwrap();
    article
}

pub fn get_articles() -> io::Result<Vec<structs::Article>> {
    let dir_path = "static/";
    let mut articles: Vec<structs::Article> = Vec::new();

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
    articles.sort();
    Ok(articles)
}
