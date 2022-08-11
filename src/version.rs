use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct Asset {
    url: String,
    id: u32,
    node_id: String,
    name: String,
    label: Option<String>,
    uploader: User,
    content_type: String,
    state: String,
    size: u32,
    download_count: u32,
    created_at: String,
    updated_at: Option<String>,
    browser_download_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Info {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: u32,
    author: User,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<Asset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
}

pub fn check_updates() -> Result<bool, Box<dyn Error>> {
    let response: Info = reqwest::blocking::Client::new()
        .get("https://api.github.com/repos/frrenzy/bem-cli/releases/latest")
        .header(USER_AGENT, "my rust app")
        .send()?
        .json()?;

    let cargo_version = env!("CARGO_PKG_VERSION");
    let gh_version = String::from(&response.tag_name[1..]);

    if cargo_version.eq(&response.tag_name[1..]) {
        println!("Your version {} is up to date.", cargo_version);

        Ok(false)

    } else {
        println!(
            "Your version is {}, while latest is {}. Consider updating via this command:",
            cargo_version, gh_version
        );

        Ok(true)
    }
}
