use askama::Template;
use chrono::format::ParseError;
use chrono::NaiveDate;
use pandoc::{InputFormat, OutputKind, Pandoc, PandocOption, PandocOutput};
use serde::Deserialize;
use std::fs;
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
    pub date: NaiveDate,
}

impl Ord for Article {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Bad Parse Somewhere lmao")
    }
}

impl PartialOrd for Article {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            if self.date < other.date {
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
        self.date.eq(&other.date)
    }
}

impl Article {
    pub fn create_template(&self) -> io::Result<EditorialTemplate> {
        let mut static_path = PathBuf::from("./");
        static_path.push(&self.content_path);

        let mut pandoc = pandoc::new();

        pandoc
            .add_input(&static_path)
            .add_option(PandocOption::MathJax(None))
            .set_output(OutputKind::Pipe);

        let output = pandoc.execute().unwrap();

        let html: String = match output {
            PandocOutput::ToBuffer(s) => s,
            PandocOutput::ToBufferRaw(bytes) => String::from_utf8(bytes).unwrap(),
            PandocOutput::ToFile(path) => fs::read_to_string(path)?,
        };

        let template = EditorialTemplate {
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.to_string(),
            content: html,
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
