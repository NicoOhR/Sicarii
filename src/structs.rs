use askama::Template;
use chrono::prelude::*;

pub struct Article {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub path: String,
    pub date: NaiveDate,
}

#[derive(Template)]
#[template(path = "editorial.html")]
pub struct EditorialTemplate<'a> {
    pub title: &'a String,
    pub author: &'a String,
    pub date: &'a String,
    pub content: &'a String,
}
