//! Handles command-line argument parsing for the Blindfold application.
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "Blindfold",
    version = env!("CARGO_PKG_VERSION"),
    author = "Eoin McMahon",
    about = "Generator of .gitignore files using gitignore.io"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum FormatOption {
    Plain,
    Table,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List available templates
    List {
        #[arg(short, long, default_value = "table")]
        format: FormatOption,
    },

    /// Generate a .gitignore
    Generate {
        #[arg(required = true)]
        languages: Vec<String>,

        #[arg(short, long, default_value = ".")]
        directory: String,

        #[arg(short, long)]
        append: bool,
    },
}
