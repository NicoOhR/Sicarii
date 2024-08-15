use askama::Template;
use chrono::prelude::*;
use markdown;
use std::fs::File;
use std::io::{self, Read};

pub struct Article {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub content_path: String,
    pub link: String,
    pub date: NaiveDate,
}

impl Article {
    pub fn create_template(&self) -> io::Result<EditorialTemplate> {
        let mut file = File::open(&self.content_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let html = markdown::to_html(&contents);

        let template = EditorialTemplate {
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.to_string(),
            content: markdown::to_html(&contents),
        };
        println!("{html}");
        Ok(template)
    }
}
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub articles: &'a [Article],
}
#[derive(Template)]
#[template(path = "editorial.html")]
pub struct EditorialTemplate {
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String,
}
