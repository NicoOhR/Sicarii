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
Unsurpisingly, the ```Article``` struct is mostly strings with a ```NaiveDate``` field, from which the ```Ord``` trait is implemented so the articles are rendered to the final site in chronological order. After all the metadata is in a neat and organized array, the articles can be written into a file directly:

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

The markdown-to-html work is done by Pandoc, both because I already have all of the haskell dependencies on my machine, and it also handles $\rm\LaTeX$ best out of any rust markdown parsers I've seen. Inevitably I think I will need to make my own parser, since I keep thinking of new ways to make the site even more pretentious looking, and github-style markdown is becoming too limiting.


