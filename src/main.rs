use reqwest;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileRes {
    name: String,
    download_url: Option<String>,
}

fn build_file_map(res: &str) -> HashMap<String, Option<String>> {
    let mut file_map: HashMap<String, Option<String>> = HashMap::new();

    // parse json response to extract name and download link
    let all_files: Vec<FileRes> = serde_json::from_str(res).unwrap();

    // filter out non-gitignore files
    let gitignore_files: Vec<&FileRes> = all_files
        .iter()
        .filter(|file| file.name.contains("gitignore"))
        .collect();

    // Insert name and url into hashmap
    for file in gitignore_files {
        let name = file.name.clone().replace(".gitignore", "");
      file_map.insert(name, file.download_url.clone());
    }

    return file_map;
}

// performs a http GET request using the reqwest crate
fn http_get(url: &str) -> String {
    let response = reqwest::get(url)
        .expect("Error getting file")
        .text()
        .expect("Could not parse Text");

    return response;
}

fn main() {
    // github api get request to list the gitignore repository files
    let repo_contents: String = http_get("https://api.github.com/repos/github/gitignore/contents");

    let file_map: HashMap<String, Option<String>> = build_file_map(&repo_contents);

    println!("{:?}", file_map)
}
