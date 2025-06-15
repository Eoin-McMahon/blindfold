mod cli;
mod client;
mod constants;
mod output;
mod service;

use std::path::PathBuf;

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
    let client = GitIgnoreIOClient::new(API_URL);
    let gitignore_service = GitIgnoreService::new(client);

    match args.command {
        Commands::List { format } => {
            if let Some(templates) = gitignore_service.list_templates().await {
                let template_outputter = TemplateOutput;
                match format {
                    FormatOption::Plain => template_outputter.write_list(templates),
                    FormatOption::Table => template_outputter.write_table(templates),
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

            if let Some(gitignore_contents) = gitignore_service.get_gitignore_contents(&langs).await
            {
                file_outputter.write(gitignore_contents, append, &output_path)?;
            } else {
                eprintln!("Error fetching gitignore contents")
            }
            return Ok(());
        }
    }
}
