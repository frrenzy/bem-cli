use std::io;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, fs::File};
use std::process::Command;

fn main() -> io::Result<()> {
    let big_dir = ".";
    let args: Vec<String> = env::args().collect();
    let path = Path::new(big_dir).canonicalize().unwrap();

    match args.len() {
        1 => {
            eprintln!("Error: missing command");
            help();
        },
        2 => {
            match &args[1][..] {
                "update" => update()?,
                "create" => {
                    create_bem(&path)?;
                    loop {
                        println!("Do you want to install dependencies? [yn]");

                        let mut answer = String::new();

                        io::stdin()
                            .read_line(&mut answer)
                            .expect("Failed to read answer");

                        match &answer.trim()[..] {
                            "y" => {
                                install()?;
                                break
                            },
                            "n" => break,
                            _ => println!("Answer should be y or n"),
                        }
                    }
                },
                "version" => println!("{}", env!("CARGO_PKG_VERSION")),
                _ => {
                    eprintln!("Error: invalid command");
                    help();
                },
            }
        },
        3 => {
            match &args[1][..] {
                "create" => {
                    match &args[2][..] {
                        "-i" => create_bem_install(&path)?,
                        "--install" => create_bem_install(&path)?,
                        _ => {
                            eprintln!("Error: invalid command");
                            help();
                        },
                    }
                },
                _ => {
                    eprintln!("Error: invalid command");
                    help();
                },
            }
        },
        _ => {
            eprintln!("Error: invalid command");
            help();
        },
    }


    Ok(())
}

fn update() -> io::Result<()> {
    let output = Command::new("zsh")
        .arg("-c")
        .arg("$(curl -fsSL https://raw.github.com/frrenzy/bem-cli/master/update.sh)")
        .output()
        .expect("Unable to update :(. Reinstall via install command: https://github.com/frrenzy/bem-cli");

    println!("Successfully updated! New version {:?}", output.stdout);

    Ok(())
}

fn create_bem_install(dir: &Path) -> io::Result<()> {
    create_bem(&dir)?;

    install()?;

    Ok(())
}

fn install() -> io::Result<()> {
    Command::new("npm")
        .arg("install")
        .spawn()
        .expect("Failed to install dependencies");

    Ok(())
}

fn create_bem(dir: &Path) -> io::Result<()> {
    generate_dotfiles(&dir)?;

    generate_directories(&dir)?;

    generate_boilerplate(&dir)?;

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
    fs::create_dir(&dir.join("src"))?;
    let bem_struct: [&str; 5] = ["blocks", "images", "pages", "components", "vendor"];
    for folder in bem_struct {
        fs::create_dir(&dir.join("src").join(&folder))?;
    }

    Ok(())
}

fn generate_boilerplate(dir: &Path) -> io::Result<()> {
    let mut file = File::create(dir.join("README.md"))?;
    let bytes = include_bytes!("assets/README.md");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("postcss.config.js"))?;
    let bytes = include_bytes!("assets/postcss.config.js");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("package.json"))?;
    let bytes = include_bytes!("assets/package.json");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("babel.config.js"))?;
    let bytes = include_bytes!("assets/babel.config.js");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("webpack.config.js"))?;
    let bytes = include_bytes!("assets/webpack.config.js");
    file.write_all(bytes)?;


    let mut file = File::create(dir.join("src/index.html"))?;
    let bytes = include_bytes!("assets/index.html");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("src/pages/index.css"))?;
    let bytes = include_bytes!("assets/index.css");
    file.write_all(bytes)?;

    let mut file = File::create(dir.join("src/vendor/normalize.css"))?;
    let bytes = include_bytes!("assets/normalize.css");
    file.write_all(bytes)?;

    let _file = File::create(dir.join("src/components/index.js"))?;
    let bytes = include_bytes!("assets/index.js");
    file.write_all(bytes)?;

    Ok(())
}

fn help() {
    println!("usage:
bem version
    Shows version.
bem update
    Downloads new version.
bem create <option>
    Generates BEM project in current empty folder.
    Optional argument:
        -i, --install: install dependencies automatically");
}
