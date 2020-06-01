use std::env;
mod lib;
use std::collections::HashMap;

// api end point for the gitignore templates repository
const API_URL: &str = "https://api.github.com/repos/toptal/gitignore/contents/templates?ref=master";

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();

    // perform a get request to list the gitignore repository files
    let repo_contents: String = lib::http_get(API_URL);
    let file_map: HashMap<String, String> = lib::build_file_map(&repo_contents);

    // ignore first argument
    for language in args[1..].iter() {
        // get gitignore template for each CLA
        println!("Language requested: {}", language);    
        let ignore_file: String = lib::get_ignore_file(&file_map, language);
        println!("{}", ignore_file);
    }
}
