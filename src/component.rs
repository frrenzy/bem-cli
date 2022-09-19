pub fn pascal_to_kebab(s: &str) -> String {
    let mut s2 = String::new();
    for letter in s.chars() {
        let char = if letter.is_ascii_lowercase() || letter.is_ascii_digit() {
            letter
        } else if letter.eq(&'_') {
            '-'
        } else {
            if s2.len() > 0 {
                s2.push('-');
            }
            letter.to_ascii_lowercase()
        };
        s2.push(char)
    }
    s2
}

pub fn pascal_to_camel(s: &str) -> String {
    let mut s2 = String::new();
    for letter in s.chars() {
        if s2.len() > 0 {
            s2.push(letter)
        } else {
            s2.push(letter.to_ascii_lowercase())
        }
    }
    s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_to_kebab() {
        assert_eq!(pascal_to_kebab("AppHeader"), "app-header");
        assert_eq!(pascal_to_kebab("appHeader"), "app-header");
        assert_eq!(pascal_to_kebab("Ap2pHeader"), "ap2p-header");
        assert_eq!(pascal_to_kebab("App_header"), "app-header");
    }

    #[test]
    fn test_pascal_to_camel() {
        assert_eq!(pascal_to_camel("AppHeader"), "appHeader");
    }
}
