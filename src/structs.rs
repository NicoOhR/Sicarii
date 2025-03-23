use askama::Template;
use chrono::format::ParseError;
use chrono::NaiveDate;
use pulldown_cmark;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Article {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub content_path: String,
    pub link: String,
    pub date: String,
}
impl Ord for Article {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Bad Parse Somewhere lmao")
    }
}

impl PartialOrd for Article {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_date = NaiveDate::parse_from_str(&self.date.trim(), "%Y-%m-%d").unwrap();
        let other_date = NaiveDate::parse_from_str(&other.date.trim(), "%Y-%m-%d").unwrap();
        if self.eq(other) {
            Some(std::cmp::Ordering::Equal)
        } else {
            if self_date < other_date {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Greater)
            }
        }
    }
}

impl Eq for Article {}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        //fix unsafe unwrap
        let mut self_date = NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").unwrap();
        let mut other_date = NaiveDate::parse_from_str(&other.date, "%Y-%m-%d").unwrap();
        self_date == other_date
    }
}

impl Article {
    pub fn create_template(&self) -> io::Result<EditorialTemplate> {
        let mut static_path = PathBuf::from("static/");
        static_path.push(&self.content_path);
        let mut file = File::open(static_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        //github style MD options
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);

        //parser-per-article seems wasteful, should probably move to it's own struct
        let md_parser = pulldown_cmark::Parser::new_ext(&contents, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, md_parser);

        let template = EditorialTemplate {
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.to_string(),
            content: html_output,
        };

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
