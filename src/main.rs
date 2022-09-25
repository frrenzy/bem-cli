mod cli_io;
mod component;
mod version;

use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    let big_dir = ".";
    let args: Vec<String> = env::args().collect();
    let path = Path::new(big_dir).canonicalize()?;

    match args.len() {
        1 => {
            eprintln!("Error: missing command");
            help();
        }
        2 => match &args[1][..] {
            "update" => handle_version()?,
            "create" => {
                create_bem(&path)?;

                if cli_io::get_install_flag()? {
                    let manager = cli_io::get_package_manager()?;
                    install(manager)?;
                }
            }
            "version" => handle_version()?,
            _ => {
                eprintln!("Error: invalid command");
                help();
            }
        },
        3 => match &args[1][..] {
            "create" => match &args[2][..] {
                "-i" => create_bem_install(&path)?,
                "--install" => create_bem_install(&path)?,
                _ => {
                    eprintln!("Error: invalid command");
                    help();
                }
            },
            "component" => handle_component_create(&path, &args[2], "css", "func")?,
            "class-component" => handle_component_create(&path, &args[2], "css", "class")?,
            _ => {
                eprintln!("Error: invalid command");
                help();
            },
        },
        4 => match &args[1][..] {
            "component" => match &args[3][..] {
                "--sass" => handle_component_create(&path, &args[2], "sass", "func")?,
                "--scss" => handle_component_create(&path, &args[2], "scss", "func")?,
                _ => {
                    eprintln!("Error: invalid command. Use --sass or --scss option to specify CSS preprocessor type");
                    help();
                }
            },
            "class-component" => match &args[3][..] {
                "--sass" => handle_component_create(&path, &args[2], "sass", "class")?,
                "--scss" => handle_component_create(&path, &args[2], "scss", "class")?,
                _ => {
                    eprintln!("Error: invalid command. Use --sass or --scss option to specify CSS preprocessor type");
                    help();
                }
            },
            _ => {
                eprintln!("Error: invalid command");
                help();
            }
        },
        _ => {
            eprintln!("Error: invalid command");
            help();
        }
    }

    Ok(())
}

fn handle_component_create(dir: &Path, name: &str, css: &str, component_type: &str) -> io::Result<()> {
    let kebab_name = component::pascal_to_kebab(name);
    let camel_name = component::pascal_to_camel(name);
    let dir = dir.join("src").join("components").join(&kebab_name[..]);
    fs::create_dir(&dir)?;

    let _module = File::create(dir.join(format!("{kebab_name}.module.{css}")))?;

    let mut index = File::create(dir.join("index.js"))?;
    index.write_all(format!("export {{ default }} from './{kebab_name}'\n").as_bytes())?;

    let mut component = File::create(dir.join(format!("{kebab_name}.jsx")))?;
    match component_type {
        "func" => component.write_all(
        format!("import {camel_name}Styles from './{kebab_name}.module.{css}'\n\nconst {name} = props => {{\n\n}}\n\nexport default {name}\n").as_bytes(),
    )?,
        "class" => component.write_all(
            format!("import React from 'react'\nimport {camel_name}Styles from './{kebab_name}.module.{css}'\n\nclass {name} extends React.Component {{\n\tconstructor(props) {{\n\t\tsuper(props)\n\t}}\n\n\trender() {{\n\n\t}}\n}}\n\nexport default {name}\n").as_bytes(),
        )?,
        _ => {}
    }

    Ok(())
}

fn handle_version() -> Result<(), Box<dyn Error>> {
    if version::check_updates()? {
        println!("zsh -c \"$(curl -fsSL https://raw.github.com/frrenzy/bem-cli/master/update.sh)\"")
    };

    Ok(())
}

fn create_bem_install(dir: &Path) -> io::Result<()> {
    create_bem(dir)?;

    let manager = cli_io::get_package_manager()?;
    install(manager)?;

    Ok(())
}

fn install(manager: &str) -> io::Result<()> {
    println!("Installing dependencies:");

    Command::new(manager)
        .arg("install")
        .status()
        .expect("Failed to install dependencies");

    Ok(())
}

fn create_bem(dir: &Path) -> io::Result<()> {
    Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialize repository");

    generate_dotfiles(dir)?;

    generate_directories(dir)?;

    generate_boilerplate(dir)?;

    println!("Done!");

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
        fs::create_dir(&dir.join("src").join(folder))?;
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

    let mut file = File::create(dir.join("src/pages/index.js"))?;
    let bytes = include_bytes!("assets/index.js");
    file.write_all(bytes)?;

    Ok(())
}

fn help() {
    println!(
        "\nusage:
bem version
    Shows version.
bem update
    Checks for updates.
bem create <option>
    Generates BEM project in current empty folder.
    Optional argument:
        -i, --install: install dependencies automatically.
bem component <name> <option>
    Generates React component with specified name in current/src directory. No preprocessors are used by default.
    Optional argument:
        --sass: create SASS-module for component
        --scss: create SCSS-module for component
bem class-component <name> <option>
    Generates React class-component with specified name in current/src directory. No preprocessors are used by default.
    Optional argument:
        --sass: create SASS-module for component
        --scss: create SCSS-module for component"
    );
}
