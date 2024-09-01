use crate::structs;
use chrono::Local;

pub fn get_articles() -> Vec<structs::Article> {
    vec![structs::Article {
        title: String::from("Side Quest Sicarii"),
        subtitle: String::from("A write up of doing front end the wrong way"),
        author: String::from("Nico O."),
        content_path: String::from("assets/sicarii/sicarii.md"),
        link: String::from("articles/sicarii.html"),
        date: Local::now().date_naive(),
    }]
}
