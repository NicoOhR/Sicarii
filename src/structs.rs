use askama::Template;
use chrono::format::ParseError;
use chrono::NaiveDate;
use kuchikiki::traits::*;
use pandoc::{InputFormat, OutputKind, Pandoc, PandocOption, PandocOutput};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::{highlighted_html_for_string, ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

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
            .add_option(PandocOption::NoHighlight)
            .set_output(OutputKind::Pipe);

        let output = pandoc.execute().unwrap();

        let html: String = match output {
            PandocOutput::ToBuffer(s) => s,
            PandocOutput::ToBufferRaw(bytes) => String::from_utf8(bytes).unwrap(),
            PandocOutput::ToFile(path) => fs::read_to_string(path)?,
        };

        let doc = kuchikiki::parse_html().one(html);
        let mut matches: Vec<_> = doc.select("pre").unwrap().collect();
        for css_match in matches {
            let node = css_match.as_node();
            if let Some(element) = node.as_element() {
                let attributes = &element.attributes.borrow();
                if let Some(lang) = attributes.get("class") {
                    let mut code = String::new();
                    for child in node.inclusive_descendants().text_nodes() {
                        code.push_str(&child.borrow());
                    }
                    let ss = SyntaxSet::load_defaults_newlines();

                    let syntax = ss
                        .find_syntax_by_token(lang)
                        .or_else(|| ss.find_syntax_by_name(&lang))
                        .unwrap_or_else(|| ss.find_syntax_plain_text());
                    let theme = ThemeSet::get_theme(Path::new("./src/Gruvbox-N.tmTheme")).unwrap();
                    let output_html =
                        highlighted_html_for_string(&code, &ss, syntax, &theme).unwrap();
                    let new_pre_html = format!(r#"<pre class="{}">{}</pre>"#, lang, output_html);
                    let fragment = kuchikiki::parse_html().one(new_pre_html);
                    node.insert_after(fragment);
                    node.detach();
                }
            }
        }

        let output = doc.to_string();
        let template = EditorialTemplate {
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.to_string(),
            content: output,
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
