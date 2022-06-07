use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, fs::File};

fn main() -> io::Result<()> {
    let big_dir = ".";

    let path = Path::new(big_dir).canonicalize().unwrap();

    generate_dotfiles(&path)?;

    generate_directories(&path)?;

    generate_boilerplate(&path)?;

    Ok(())
}

fn generate_dotfiles(dir: &Path) -> io::Result<()> {
    let _file = File::create(dir.join(".nojekyll"))?;

    let mut file = File::create(dir.join(".gitignore"))?;
    let bytes = include_bytes!("assets/.gitignore");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join(".editorconfig"))?;
    let bytes = include_bytes!("assets/.editorconfig");
    file.write_all(bytes)?;

    Ok(())
}

fn generate_directories(dir: &Path) -> io::Result<()> {
    let bem_struct: [&str; 5] = ["blocks", "images", "pages", "scripts", "vendor"];
    for folder in bem_struct {
        fs::create_dir(&dir.join(&folder))?;
    }

    Ok(())
}

fn generate_boilerplate(dir: &Path) -> io::Result<()> {
    let mut file = File::create(dir.join("index.html"))?;
    let bytes = include_bytes!("assets/index.html");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("README.md"))?;
    let bytes = include_bytes!("assets/README.md");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("pages/index.css"))?;
    let bytes = include_bytes!("assets/index.css");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("vendor/normalize.css"))?;
    let bytes = include_bytes!("assets/normalize.css");
    file.write_all(bytes)?;

    let _file = File::create(dir.join("scripts/index.js"))?;

    Ok(())
}
