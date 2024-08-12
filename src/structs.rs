use askama::Template;
use chrono::prelude::*;
use std::fs;
use std::io;

pub struct Article {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub path: String,
    pub date: NaiveDate,
}

impl Article {
    pub fn create_template(self) -> io::Result<EditorialTemplate> {
        let template = EditorialTemplate {
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.to_string(),
            content: fs::read_to_string(self.path)?,
        };

        Ok(template)
    }
}

#[derive(Template)]
#[template(path = "editorial.html")]
pub struct EditorialTemplate {
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String,
}
