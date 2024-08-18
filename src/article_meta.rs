use crate::structs;
use chrono::Local;

pub fn get_articles() -> Vec<structs::Article> {
    vec![structs::Article {
        title: String::from("You should guage out your organs"),
        subtitle: String::from("The reasonable arguments of Deleuze"),
        author: String::from("Art, Culture, Philosophy"),
        content_path: String::from("assets/organs/organs.md"),
        link: String::from("articles/organs.html"),
        date: Local::now().date_naive(),
    }]
}
