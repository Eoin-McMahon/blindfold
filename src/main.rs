mod cli;
mod client;
mod constants;
mod log;
mod output;
mod service;
use reqwest::Client as HTTPClient;
mod typo;
use typo::check_typos;

use std::{io::stdout, path::PathBuf};

use client::GitIgnoreIOClient;
use service::GitIgnoreService;

use clap::Parser;
use cli::{Cli, Commands};
use constants::API_URL;

use crate::{
    cli::FormatOption,
    log::{log_done, log_error, log_info},
    output::{FileOutput, Output},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = GitIgnoreIOClient::new(API_URL, HTTPClient::new());
    let gitignore_service = GitIgnoreService::new(client);
    let outputter = FileOutput;

    // Lock stdout for duration
    let stdout = stdout();
    let mut handle = stdout.lock();

    let templates = match gitignore_service.list_templates().await {
        Some(templates) => templates,
        None => {
            log_error(
                String::from("Unable to fetch available .gitignore templates"),
                &mut handle,
            )?;
            return Ok(()); // exit early if templates couldn't be fetched
        }
    };

    match args.command {
        Commands::List { format } => {
            log_info(
                String::from("Fetching available .gitignore templates"),
                &mut handle,
            )?;
            match format {
                FormatOption::Plain => outputter.write_list(templates, handle)?,
                FormatOption::Table => outputter.write_table(templates, handle)?,
            }
            return Ok(());
        }
        Commands::Generate {
            languages,
            directory,
            append,
        } => {
            let langs: Vec<&str> = languages.iter().map(|s| s.as_str()).collect();
            let langs_str = &langs.join(", ");

            // Create output path
            let mut output_path = PathBuf::from(directory);
            output_path.push(".gitignore");

            log_info(
                format!("Fetching .gitignore contents for {}", langs_str),
                &mut handle,
            )?;

            if check_typos(&mut handle, &langs, &templates) {
                // If typos exit early
                return Ok(());
            }

            let gitignore_contents = match gitignore_service.get_gitignore_contents(&langs).await {
                Some(gitignore) => gitignore,
                None => {
                    log_error(
                        format!("Failed to fetch .gitignore contents for {}", langs_str),
                        &mut handle,
                    )?;
                    return Ok(());
                }
            };

            if let Err(e) = outputter.write_gitignore(gitignore_contents, append, &output_path) {
                log_error(
                    format!(
                        "Failed to write .gitignore to {}: {}",
                        output_path.display(),
                        e
                    ),
                    &mut handle,
                )?;
                return Ok(());
            }

            log_done(
                format!(
                    "Successfully wrote .gitignore for {} to {}",
                    langs_str,
                    output_path.display()
                ),
                &mut handle,
            )?;
            Ok(())
        }
    }
}
