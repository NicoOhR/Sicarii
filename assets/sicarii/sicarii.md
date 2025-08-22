I always struggled with web development which for a software-guy (tm) is a bit of a personal tragedy. I have a hard time getting interested in the semantics and philosophies of different web framework, why are there so many ways to interact with the DOM? Why is the website thirsty? Still, without a personal website, what am I supposed to link to on linkedin?

In a fit of "not made here" syndrome I figured that, all an SSG does is translate one markup language to another and push it into plain text templates, I might as well do it myself. I decided to use Rust (because I use Rust for everything at this point), with the Askama templating library. 

Each article is kept in its own directory with a markdown file with the contents of the article and a toml file with the metadata of the article, as well as and any other assets which it need to include.

```Rust
pub fn get_articles() -> io::Result<Vec<structs::Article>> {
    let dir_path = "./assets/";
    let mut articles: Vec<structs::Article> = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "toml" {
                    articles.push(read_toml(&fs::read_to_string(path)?));
                }
            }
        }
    }
    articles.sort();
    Ok(articles)
}
```
Unsurprisingly, the ```Article``` struct is mostly strings with a ```NaiveDate``` field, from which the ```Ord``` trait is implemented so the articles are rendered to the final site in chronological order. After all the metadata is in a neat and organized array, the articles can be written into a file directly:

```Rust 
fn render_to_file(content: String, path: &str) -> io::Result<()> {
    println!("Created Content Path");
    let mut content_path = PathBuf::from("./site/");
    content_path.push(path);
    println!("{content_path:?}");

    if let Some(parent) = content_path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = File::create(content_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
```

The markdown-to-html work is done by Pandoc, both because I already have all of the haskell dependencies on my machine, and it also handles $\rm\LaTeX$ best out of any rust markdown parsers I tried out. Code highlighting is done through [syntect](https://github.com/trishume/syntect), a charming library which lets you use sublime syntax definitions to highlight code. I was originaly using highlightjs, but ultimately couldn't help but further oxidize this project. 

The ```markdown -> pandoc -> syntect-> html``` pipeline was not as smooth as I would have liked, and required a good bit of elbow grease to get to a well behaving state. In a nutshell after pandoc generated the initial HTML, I used [kuchikiki](https://github.com/brave/kuchikiki) to find and the ```<code>``` and feed them to syntect, then replacing the original element with what syntect produces. Inevitably I think I'd like to make my own parser, since kuchikiki pulls in html5ever, which, while being a great library, adds a significant amount of additional dependencies to this project; which I feel already has too many.

```Rust
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
                    let syntax = ss.find_syntax_by_token(lang).unwrap();
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
```

This project gave me some appreciation for the static site generators
which have taken over the tech-blogging space. It's easy to chalk up much
of what they do to "translating one markup language to another" but
a great deal of thought goes into making the end user experience as easy
anand flexible as possible. 
