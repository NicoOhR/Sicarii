
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
