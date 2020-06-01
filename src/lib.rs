use std::collections::HashMap;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRes {
    name: String,
    download_url: Option<String>,
}

pub fn build_file_map(res: &str) -> HashMap<String, String> { 
    // parse json response to extract name and download link into FileRes struct
    let all_files: Vec<FileRes> = serde_json::from_str(res).unwrap();

    // filter out non-gitignore files
    let gitignore_files: Vec<&FileRes> = all_files
        .iter()
        .filter(|file| file.name.contains("gitignore"))
        .collect();

    // destructure vec of structs to vec of tuples in form (name, url)
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
pub fn destructure_to_tup(file_struct: &FileRes) -> (String, String) {
    // format name to be language name lowercased
    let name:String = file_struct.name
        .clone()
        .replace(".gitignore", "")
        .to_lowercase();
    

    let mut url:String = String::from("");

    if let Some(download_url) = &file_struct.download_url  {
        url.push_str(download_url);
    }

    return (name, url);
}

// performs a http GET request using the reqwest crate
pub fn http_get(url: &str) -> String {
    let response = reqwest::get(url)
        .expect("Error getting file")
        .text()
        .expect("Could not parse Text");

    return response;
}

// make http get request for the specified template and return it as a string
pub fn get_ignore_file(file_map: &HashMap<String, String>, lang: &str) -> String {
    let mut response: String = String::from("");
    let file_url: Option<&String> = file_map.get(lang);
    
    if let Some(file) = file_url {
        response.push_str(&http_get(&file));
    }

    return response;
}