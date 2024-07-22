use std::{env, fs, io::Cursor, path::PathBuf};

use reqwest::blocking::{get, Client};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Release {
    name: String,
    assets: [Asset; 2],
}

#[derive(Debug, Deserialize)]
struct Asset {
    browser_download_url: String,
}

fn main() {
    let client = Client::builder()
        .user_agent("katex-rquickjs")
        .build()
        .unwrap();

    let Release { name: latest_version, assets: [_, Asset { browser_download_url }] } = client
        .get("https://api.github.com/repos/KaTeX/KaTeX/releases/latest")
        .send()
        .expect("GET request to 'https://api.github.com/repos/KaTeX/KaTeX/releases/latest' failed")
        .json::<Release>()
		.expect("Failed to parse JSON response from 'https://api.github.com/repos/KaTeX/KaTeX/releases/latest'");

    let katex_release_file = get_target_dir().join("katex-release.txt");
    match fs::read_to_string(&katex_release_file) {
        // No change in version since the last build
        Ok(current_version) if &current_version == &latest_version => (),
        // Latest version has changed, or this is a fresh build
        Ok(_) | Err(_) => {
            fs::write(katex_release_file, &latest_version).unwrap();
            get_katex(browser_download_url);
        }
    }
}

fn get_katex(browser_download_url: String) {
    let zip_data = get(browser_download_url)
        .expect("Failed to fetch katez release zip archive")
        .bytes()
        .unwrap();
    let katex_dir = get_target_dir().join("katex-js-release");
	if katex_dir.exists() {
		fs::remove_dir_all(&katex_dir).unwrap();
	}
    zip_extract::extract(Cursor::new(zip_data), &katex_dir, true)
        .expect("Failed to extract katex release zip archive");
}

fn get_target_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
}
