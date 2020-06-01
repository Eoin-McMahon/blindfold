use std::env;
use std::collections::HashMap;
use reqwest;
use serde::{Serialize, Deserialize};

// api end point for the gitignore templates repository
const API_URL: &str = "https://api.github.com/repos/toptal/gitignore/contents/templates?ref=master";

#[derive(Serialize, Deserialize, Debug)]
struct FileRes {
    name: String,
    download_url: Option<String>,
}

fn build_file_map(res: &str) -> HashMap<String, String> { 
    // parse json response to extract name and download link into FileRes struct
    let all_files: Vec<FileRes> = serde_json::from_str(res).unwrap();

    // filter out non-gitignore files
    let gitignore_files: Vec<&FileRes> = all_files
        .iter()
        .filter(|file| file.name.contains("gitignore"))
        .collect();

    // destructure vec of struct to vec of tuples in form (name, url)
    let destructured: Vec<(String, String)> = gitignore_files
        .iter()
        .map(|file| destructure_to_tup(file))
        .collect();

    // collect vector of tuples into a hashmap
    let file_map: HashMap<String, String> = destructured
        .into_iter()
        .collect();

    return file_map;
}

// destructure FileRes struct to a tuple of its fields
fn destructure_to_tup(file_struct: &FileRes) -> (String, String) {
    // format name to be language name lowercased
    let name:String = file_struct.name
        .clone()
        .replace(".gitignore", "")
        .to_lowercase();
    

    let mut url:String = String::from("");

    if let Some(download_url) = &file_struct.download_url  {
        url.push_str(download_url);
    }

    let tuple: (String, String) = (name, url);

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

// make http get request for the specified template and return it as a string
fn get_ignore_file(file_map: &HashMap<String, String>, lang: &str) -> String {
    let mut response: String = String::from("");
    let file_url: Option<&String> = file_map.get(lang);
    
    if let Some(file) = file_url {
        response.push_str(&http_get(&file));
    }

    return response;
}


fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();

    // perform a get request to list the gitignore repository files
    let repo_contents: String = http_get(API_URL);
    let file_map: HashMap<String, String> = build_file_map(&repo_contents);

    // ignore first argument
    for language in args[1..].iter() {
        // get gitignore template for each CLA
        println!("Language requested: {}", language);    
        let ignore_file: String = get_ignore_file(&file_map, language);
        println!("{}", ignore_file);
    }
}
