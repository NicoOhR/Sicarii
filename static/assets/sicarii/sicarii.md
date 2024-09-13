
Generally, I don't like web development. Tragically, the most effective way to show anything in tech will almost always be, web development. It's mostly the convinience factor, but there's something very appealing about easily distributing your project in an interactive and responsive manner, which happens to be the thesis (in my mind) of web apps. Web development framework's have always been unwieldly to me, and I just struggle to get interested in the semantics of frameworks. 

To my delight, in the last couple of years a sort of *post-modern* web app tech stack has emerged. Using HTMX and some templating program (typically via a library in your language of choice), you can avoid the "web" part of "web app development" almost entierly. This site, which is not yet a web app, is an experiment in building a website "in the wrong way".

I decided to use Rust (because I use Rust for everything at this point), with the Askama templating library. 

Rendering to file happpens like so:

```rust 


fn render_to_file(content: String, path: &String) -> io::Result<()> {
    let mut content_path = PathBuf::from("./static/");
    content_path.push(path);

    println!("{content_path:?}");

    let mut file = File::create(content_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

```
