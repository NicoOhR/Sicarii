
I (generally) don't like web development. Tragically, the most effective way to show anything in tech will almost always be, web development. It's mostly the convinience factor, but there's something very appealing about easily distributing your project in an interactive and responsive manner, which happens to be the thesis (in my mind) of web apps. Web development framework's have always been unwieldly to me, and I just struggle to get interested in the semantics of frameworks. 

To my delight, in the last couple of years a sort of *post-modern* web app tech stack has emerged. Using HTMX and some templating program (typically via a library in your language of choice), you can avoid the "web" part of "web app development" almost entierly. This site, which is not yet a web app, is an experiment in building a website "in the wrong way".

I decided to use Rust (because I use Rust for everything at this point), with the Askama templating library. 

Rendering to file happpens like so:

```rust 
fn render_to_file(content: String, path: &String) -> io::Result<()> {
    let mut content_path = PathBuf::from("./static/");
    content_path.push(path);

    let mut file = File::create(content_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

```

This function is used in main

```rust
render_to_file(homepage.render().unwrap(), &String::from("index.html"))?;

for article in articles.iter() {
    render_to_file(article.create_template()?.render().unwrap(), &article.link)?;
}
```

Each article is a struct made of the meta data of the article. Originally I hada function returning a long list of the article structs, and it worked... fine?I wanted to move the information on an article a little closer to the article itself so I switched to a TOML file in the same directory as the markdown file.

```rust
fn read_toml(s: &str) -> structs::Article {
    let article: structs::Article = toml::from_str(s).unwrap();
    article
}

pub fn get_articles() -> io::Result<Vec<structs::Article>> {
    let dir_path = "static/";
    let mut articles: Vec<structs::Article> = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "toml" {
                    articles.push(read_toml(&fs::read_to_string(path)?))
                }
            }
        }
    }

    Ok(articles)
}
```

So far so simple, not too bad for about a weekend of hacking worths of work. The motivation for all of this is that this lends itself very easily to extending it self to becoming anything it needs be. Very quickly Sicarii can be made into a webserver, orchestrated with other contained programs to display and serve whatever is needed of it.
