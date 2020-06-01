use reqwest;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileRes {
    name: String,
    download_url: Option<String>,
}

fn build_file_map(res: &str) -> HashMap<String, Option<String>> {
    // parse json response to extract name and download link
    let all_files: Vec<FileRes> = serde_json::from_str(res).unwrap();

    // filter out non-gitignore files
    let gitignore_files: Vec<&FileRes> = all_files
        .iter()
        .filter(|file| file.name.contains("gitignore"))
        .collect();

    // destructure vec of struct to vec of tuples in form (name, url)
    let destructured: Vec<(String, Option<String>)>= gitignore_files
        .iter()
        .map(|file| destructure_to_tup(file))
        .collect();

    // collect vector of tuples into a hashmap
    let file_map: HashMap<String, Option<String>> = destructured
        .into_iter()
        .collect();

    return file_map;
}

// destructure FileRes struct to a tuple of its fields
fn destructure_to_tup(file_struct: &FileRes) -> (String, Option<String>) {
    // format name to be language name lowercased
    let name:String = file_struct.name
        .clone()
        .replace("gitignore", "")
        .to_lowercase();
    
    let url:Option<String> = file_struct.download_url
        .clone();

    let tuple: (String, Option<String>) = (name, url);

    return tuple;
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
