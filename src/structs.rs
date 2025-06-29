use askama::Template;
use chrono::format::ParseError;
use chrono::NaiveDate;
use lol_html::{element, html_content::ContentType, text, HtmlRewriter, Settings};
use pandoc::{InputFormat, OutputKind, Pandoc, PandocOption, PandocOutput};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
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

        let mut html_output = Vec::new();

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    element!("pre[class]", |el| {
                        if let Some(pre_class) = el.get_attribute("class") {
                            //exctracted lang from the html block, need to find an easy way to get
                            //access to the internals of the block as text so that we can run it
                            //through syntact
                            let lang = pre_class
                                .strip_prefix("language-")
                                .or_else(|| Some(&pre_class))
                                .unwrap_or("plain");
                            println!("Found code block with language: {}", lang);
                        }
                        Ok(())
                    }),
                    text!("pre[class] *", |t| {
                        if inside_pre {
                            buffer.push_str(t.text());
                        }
                        Ok(())
                    }),
                ],
                ..Settings::default()
            },
            |chunk: &[u8]| {
                html_output.extend_from_slice(chunk);
            },
        );
        rewriter.write(html.as_bytes()).unwrap();
        rewriter.end().unwrap();
        let final_html = String::from_utf8(html_output).unwrap();
        //
        //let lang = element.value().attr("class").unwrap_or_default();
        //
        //let code = element.text().collect::<String>();
        //let ss = SyntaxSet::load_defaults_newlines();
        //let ts = ThemeSet::load_defaults();
        //
        //let syntax = ss
        //    .find_syntax_by_name(lang)
        //    .or_else(|| ss.find_syntax_by_name(&lang))
        //    .unwrap_or_else(|| ss.find_syntax_plain_text());
        //let theme = &ts.themes["base16-ocean.dark"];
        //let output_html = highlighted_html_for_string(&code, &ss, syntax, theme).unwrap();
        //
        //println!("{output_html}");
        //
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
