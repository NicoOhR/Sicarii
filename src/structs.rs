use askama::Template;
use pulldown_cmark::{html, Options, Parser};
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

impl Article {
    pub fn create_template(&self) -> io::Result<EditorialTemplate> {
        let mut static_path = PathBuf::from("static/");
        static_path.push(&self.content_path);
        let mut file = File::open(static_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        //parser-per-article seems wasteful
        //github style MD options
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_TABLES);
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
