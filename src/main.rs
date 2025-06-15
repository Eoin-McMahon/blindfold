mod cli;
mod client;
mod constants;
mod output;
mod service;
use reqwest::Client as HTTPClient;

use std::{io::stdout, path::PathBuf};

use client::GitIgnoreIOClient;
use service::GitIgnoreService;

use clap::Parser;
use cli::{Cli, Commands};
use constants::API_URL;
use output::TemplateOutput;

use crate::{
    cli::FormatOption,
    output::{FileOutput, Output},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = GitIgnoreIOClient::new(API_URL, HTTPClient::new());
    let gitignore_service = GitIgnoreService::new(client);

    match args.command {
        Commands::List { format } => {
            if let Some(templates) = gitignore_service.list_templates().await {
                let template_outputter = TemplateOutput;

                // Write output to stdout
                let stdout = stdout();
                let handle = stdout.lock();

                match format {
                    FormatOption::Plain => template_outputter.write_list(templates, handle)?,
                    FormatOption::Table => template_outputter.write_table(templates, handle)?,
                }
            } else {
                eprintln!("Error fetching available gitignore's");
            }
            return Ok(());
        }
        Commands::Generate {
            languages,
            destination,
            append,
        } => {
            let langs: Vec<&str> = languages.iter().map(|s| s.as_str()).collect();
            let output_path = PathBuf::from(destination);
            let file_outputter = FileOutput;

            let gitignore_contents = match gitignore_service.get_gitignore_contents(&langs).await {
                Some(gitignore) => gitignore,
                None => {
                    eprintln!("Error fetching gitignore contents");
                    return Ok(());
                }
            };

            if let Err(e) = file_outputter.write(gitignore_contents, append, &output_path) {
                eprintln!("Failed to write gitignore contents: {}", e);
            }

            Ok(())
        }
    }
}
