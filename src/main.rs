use clap::{App, Arg, SubCommand};
use colored::*;
use std::collections::HashMap;
mod lib;
#[cfg(test)]
mod test;

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
                        .help("Template(s) to generate gitignore for i.e Rust, Flutter, VsCode etc. WARNING: this will override any current gitignore"))
                    .arg(Arg::with_name("APPEND LANGUAGE(S)")
                        .short("a")
                        .long("append")
                        .takes_value(true)
                        .multiple(true)
                        .help("Adds template(s) to pre-existing gitignore file_map"))
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

    // unwrap arguments and generate gitignore
    let destination: &str = matches.value_of("DESTINATION").unwrap_or("./");

    // if passed list command, list and return
    if let Some(_) = matches.subcommand_matches("list") {
        lib::list_templates(file_map);
        return Ok(());
    } else if matches.is_present("LANGUAGE(S)") {
        let languages: Vec<&str> = matches.values_of("LANGUAGE(S)").unwrap().collect();
        let gitignore: String = lib::generate_gitignore_file(languages, &file_map);
        // write gitignore to file

        if !gitignore.is_empty() {
            lib::write_to_file(destination, gitignore).expect("Couldn't write to file ⚠️ ");
            return Ok(());
        }
    } else if matches.is_present("APPEND LANGUAGE(S)") {
        let additional_languages: Vec<&str> =
            matches.values_of("APPEND LANGUAGE(S)").unwrap().collect();
        let gitignore: String = lib::generate_gitignore_file(additional_languages, &file_map);

        if !gitignore.is_empty() {
            // append to existing gitignore to file
            lib::append_to_file(destination, gitignore).expect("Couldn't write to file ⚠️ ");
            return Ok(());
        }
    }

    // if no arguments are supplied, exit
    println!("{}, no gitignore to write! ⚠️", "Stopping".red());

    return Ok(());
}
