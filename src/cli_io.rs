use std::io;

#[macro_export]
macro_rules! cli {
    ($q:expr; $($arms:expr),+; $($returns:expr),+) => {{
        let mut answer = String::new();
        loop {
            println!("{}", $q);

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read answer");

            match answer.trim() {
                $($arms => break $returns,)*
                _ => continue,
            }
        }
    }};
    ($q:expr; $($arms:expr),+) => {{
        let mut answer = String::new();
        loop {
            println!("{}", $q);

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read answer");

            match answer.trim() {
                $($arms => break $arms,)*
                _ => continue,
            }
        }
    }};
}

pub fn get_package_manager() -> io::Result<&'static str> {
    let result = cli!("Do you want to use yarn or npm? [yarn/npm]"; "yarn", "npm");

    Ok(result)
}

pub fn get_install_flag() -> io::Result<bool> {
    let result = cli!("Do you want to install dependencies? [yn]"; "y", "n"; true, false);

    Ok(result)
}
