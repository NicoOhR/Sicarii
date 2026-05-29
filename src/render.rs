use crate::article::{Article, EditorialTemplate};
use kuchikiki::traits::*;
use pulldown_cmark::{Event, Options, Tag, TagEnd};
use regex::{Captures, Regex};
use std::fs;
use std::io;
use std::path::PathBuf;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

impl Article {
    pub fn resolve_content(&self) -> String {
        let mut static_path = PathBuf::from("./");
        static_path.push(&self.content_path);

        let text = fs::read_to_string(static_path).unwrap();
        let re = Regex::new(r"\{\{\{([^}]*)\}\}\}").unwrap();
        let result = re.replace_all(&text, |caps: &Captures| {
            let mut src_path = PathBuf::from("./");
            src_path.push(&caps[1]);
            fs::read_to_string(src_path).unwrap()
        });
        let mut content = result.to_string();
        Self::resolve_sidebars(&mut content)
    }
    pub fn resolve_sidebars(content: &mut String) -> String {
        let re = Regex::new(r"\$\$\$([\s\S]*?)\$\$\$").unwrap();
        let mut index = 0;
        re.replace_all(content, |caps: &Captures| {
            index += 1;
            format!(
                "<label for=\"cmake\" id=\"{}\" class=\"margin-toggle sidenote-number\">
                </label><input type=\"checkbox\" id=\"cmake\" class=\"margin-toggle\"/>
                <span class=\"sidenote\">{}</span>",
                index, &caps[1]
            )
        })
        .to_string()
    }

    fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    pub fn create_toc(events: pulldown_cmark::Parser) -> String {
        let mut items: Vec<(usize, String)> = Vec::new();
        let mut in_header = false;
        let mut curr_level = 0;
        let mut curr_text = String::new();

        for event in events {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    curr_level = level as usize;
                    curr_text.clear();
                    in_header = true;
                }
                Event::Text(str) => {
                    if in_header {
                        curr_text.push_str(&str);
                    }
                }
                Event::End(TagEnd::Heading(_)) => {
                    if in_header {
                        items.push((curr_level, curr_text.clone()));
                    }
                    in_header = false;
                    curr_level = 0;
                }
                _ => continue,
            }
        }

        let mut html = String::from("<nav class=\"toc\"><ul>\n");
        for (level, text) in &items {
            let id = Self::slugify(text);
            html.push_str(&format!(
                "  <li class=\"toc-h{}\"><a href=\"#{}\">{}</a></li>\n",
                level, id, text
            ));
        }
        html.push_str("</ul></nav>\n");
        html
    }

    pub fn create_template(&self) -> io::Result<EditorialTemplate> {
        let text = self.resolve_content();
        let options = Options::all();
        let mut html = Self::create_toc(pulldown_cmark::Parser::new_ext(&text, options));
        pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new_ext(&text, options));

        let doc = kuchikiki::parse_html().one(html);
        let matches: Vec<_> = doc
            .select("pre > code[class^=\"language-\"]")
            .unwrap()
            .collect();
        let custom_ss = SyntaxSet::load_from_folder("./themes/syntaxes").unwrap();
        let default_ss = SyntaxSet::load_defaults_newlines();

        for css_match in matches {
            let code_node = css_match.as_node();

            let class_attr = code_node
                .as_element()
                .and_then(|el| el.attributes.borrow().get("class").map(|s| s.to_string()))
                .unwrap_or_default();

            let lang = class_attr
                .split_whitespace()
                .find_map(|c| c.strip_prefix("language-"))
                .map(|l| l.trim_matches(|c: char| c == '{' || c == '}'))
                .unwrap_or("text");

            let code_text = code_node.text_contents();

            let (syntax, active_ss) = custom_ss
                .find_syntax_by_token(lang)
                .map(|s| (s, &custom_ss))
                .or_else(|| custom_ss.find_syntax_by_name(lang).map(|s| (s, &custom_ss)))
                .or_else(|| {
                    custom_ss
                        .find_syntax_by_extension(lang)
                        .map(|s| (s, &custom_ss))
                })
                .or_else(|| {
                    default_ss
                        .find_syntax_by_token(lang)
                        .map(|s| (s, &default_ss))
                })
                .or_else(|| {
                    default_ss
                        .find_syntax_by_name(lang)
                        .map(|s| (s, &default_ss))
                })
                .or_else(|| {
                    default_ss
                        .find_syntax_by_extension(lang)
                        .map(|s| (s, &default_ss))
                })
                .unwrap_or_else(|| (default_ss.find_syntax_plain_text(), &default_ss));

            let mut generator = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                active_ss,
                ClassStyle::SpacedPrefixed { prefix: "hl-" },
            );
            for line in LinesWithEndings::from(code_text.as_str()) {
                generator
                    .parse_html_for_line_which_includes_newline(line)
                    .unwrap();
            }
            let inner = generator.finalize();

            let pre_node = code_node.parent().expect("code should have a parent <pre>");

            let new_html = format!(
                r#"<div class="code-block" data-lang="{}"><pre class="hl-code code language-{}"><code>{}</code></pre></div>"#,
                lang, lang, inner
            );

            let fragment = kuchikiki::parse_html().one(new_html);
            pre_node.insert_after(fragment);
            pre_node.detach();
        }

        for selector in &["h1", "h2", "h3", "h4", "h5", "h6"] {
            if let Ok(headings) = doc.select(selector) {
                for heading in headings.collect::<Vec<_>>() {
                    let node = heading.as_node();
                    let text = node.text_contents();
                    let id = Self::slugify(&text);
                    node.as_element()
                        .unwrap()
                        .attributes
                        .borrow_mut()
                        .insert("id", id);
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
