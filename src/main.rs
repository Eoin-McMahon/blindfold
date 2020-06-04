use clap::{Arg, App, SubCommand};
use std::collections::HashMap;
use std::ops::Not;
use colored::*;
mod lib;

// API endpoint for the gitignore templates repository
const API_URL: &str = "https://api.github.com/repos/toptal/gitignore/contents/templates?ref=master";

fn main() -> std::io::Result<()> {
    let matches = App::new("Blindfold")
                    .version("1.0")
                    .author("Eoin McMahon <eoin.mcmahon.dev@gmail.com>")
                    .about("Grabs gitignore templates from gitignore.io")
                    .arg(Arg::with_name("LANGUAGE(S)")
                        .short("l")
                        .long("lang")
                        .takes_value(true)
                        .multiple(true)
                        .help("template(s) to generate gitignore for i.e Rust, Flutter, VsCode etc."))
                    .arg(Arg::with_name("DESTINATION")
                        .short("d")
                        .long("dest")
                        .help("Destination to store the gitignore file in")
                        .takes_value(true))
                    .subcommand(SubCommand::with_name("list")
                        .about("Lists all available gitignore templates"))
                    .get_matches();

    // perform a get request to list the gitignore repository files
    let repo_contents: String = lib::http_get(API_URL);
    let file_map: HashMap<String, String> = lib::build_file_map(&repo_contents);

    // if passed list command, list and return
    if let Some(_) = matches.subcommand_matches("list") {
        list_templates(file_map);
        return Ok(());
    }
    
    // if no arguments are supplied, exit
    if matches.is_present("LANGUAGE(S)").not() {
        println!(" {}, nothing to write! ⚠️", "No language supplied as argument".red());
        return Ok(());
    }

    // unwrap arguments and generate gitignore
    let destination: &str = matches.value_of("DESTINATION").unwrap_or("./");
    let languages: Vec<&str> = matches.values_of("LANGUAGE(S)").unwrap().collect();
    let gitignore: String = lib::generate_gitignore_file(languages, file_map);

    // write gitignore to file
    lib::write_file(destination, gitignore).expect("Couldn't write to file ⚠️ ");

    
    return Ok(());
}

fn list_templates(file_map: HashMap<String, String>) {
    for key in file_map.keys() {
        print!("{} \t", key);
    }
}

