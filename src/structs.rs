use askama::Template;
use chrono::format::ParseError;
use chrono::NaiveDate;
use kuchikiki::traits::*;
use pandoc::{InputFormat, OutputKind, Pandoc, PandocOption, PandocOutput};
use pulldown_cmark::Options;
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
use yaml_rust::parser::Parser;

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

        let text = fs::read_to_string(static_path).unwrap();
        let mut options = Options::empty();
        //options.insert(Options::ENABLE_MATH);
        let parser = pulldown_cmark::Parser::new_ext(&text, options);

        let mut html: String = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        // Kuchikiki parses HTML and adds syntect as needed
        let doc = kuchikiki::parse_html().one(html);
        // cmarkdown outputs display code as <pre><class ="language-lang">
        let matches: Vec<_> = doc
            .select("pre > code[class^=\"language-\"]")
            .unwrap()
            .collect();
        let ss = SyntaxSet::load_defaults_newlines();
        let theme = ThemeSet::get_theme(Path::new("./src/Gruvbox-N.tmTheme")).unwrap();

        for css_match in matches {
            let code_node = css_match.as_node();

            //get the class attributes of the node
            let class_attr = code_node
                .as_element()
                .and_then(|el| el.attributes.borrow().get("class").map(|s| s.to_string()))
                .unwrap_or_default();

            //extract the language name in the way that syntact expects
            let lang = class_attr
                .split_whitespace()
                .find_map(|c| c.strip_prefix("language-"))
                .unwrap_or("text");

            let code_text = code_node.text_contents();

            let syntax = ss
                .find_syntax_by_token(lang)
                .or_else(|| ss.find_syntax_by_name(lang))
                .unwrap_or_else(|| ss.find_syntax_plain_text());

            let highlighted = highlighted_html_for_string(&code_text, &ss, syntax, &theme).unwrap();

            let pre_node = code_node.parent().expect("code should have a parent <pre>");

            let new_html = format!(
                r#"<pre><code class="language-{}">{}</code></pre>"#,
                lang, highlighted
            );

            let fragment = kuchikiki::parse_html().one(new_html);
            pre_node.insert_after(fragment);
            pre_node.detach();
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
