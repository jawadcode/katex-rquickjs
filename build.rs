use std::{env, fs, io::Cursor, path::PathBuf};

use reqwest::blocking::{get, Client};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Release {
    assets: [Asset; 2],
}

#[derive(Debug, Deserialize)]
struct Asset {
    browser_download_url: String,
}

fn main() {
    println!("cargo:rerun-if-changed=./katex-release.txt");
    let katex_release_file = get_proj_dir().join("katex-release.txt");
    let current_release =
        fs::read_to_string(katex_release_file).expect("Missing katex-release.txt");

    let client = Client::builder()
        .user_agent("katex-rquickjs")
        .build()
        .unwrap();

    let api_endpoint =
        "https://api.github.com/repos/KaTeX/KaTeX/releases/tags/".to_string() + &current_release;
    let Release {
        assets: [_, Asset {
            browser_download_url,
        }],
    } = client
        .get(api_endpoint)
        .send()
        .expect("GET request to GitHub Releases API endpoint failed")
        .json::<Release>()
        .expect("Failed to parse JSON response from GitHub Releases API endpoint");

    get_katex(browser_download_url);
}

fn get_katex(browser_download_url: String) {
    let zip_data = get(browser_download_url)
        .expect("Failed to fetch katez release zip archive")
        .bytes()
        .unwrap();
    let katex_dir = get_proj_dir().join("katex-js-release");
    if katex_dir.exists() {
        fs::remove_dir_all(&katex_dir).unwrap();
    }
    zip_extract::extract(Cursor::new(zip_data), &katex_dir, true)
        .expect("Failed to extract katex release zip archive");
}

fn get_proj_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}
