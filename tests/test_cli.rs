use blindfold::cli::{self, FormatOption};
use clap::Parser;

/// Test that generate command is parsed correctly
#[test]
fn test_cli_parsing_generate() {
    let args = vec![
        "blindfold",
        "generate",
        "Rust",
        "Python",
        "-d",
        "my_gitignore",
        "-a",
    ];
    let cli = cli::Cli::parse_from(args);

    match cli.command {
        cli::Commands::Generate {
            languages,
            destination,
            append,
        } => {
            assert_eq!(languages, vec!["Rust", "Python"]);
            assert_eq!(destination, "my_gitignore");
            assert!(append);
        }
        _ => panic!("Expected Generate command"),
    }
}

/// Test that list command is parsed correctly
#[test]
fn test_cli_parsing_list() {
    let args = vec!["blindfold", "list", "--format", "table"];
    let cli = cli::Cli::parse_from(args);

    match cli.command {
        cli::Commands::List { format } => {
            assert_eq!(format, FormatOption::Table);
        }
        _ => panic!("Expected List command"),
    }
}
