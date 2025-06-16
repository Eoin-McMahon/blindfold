use std::fs;
use std::io::{Cursor, ErrorKind};
use std::path::{Path, PathBuf};

use blindfold::output::{FileOutput, Output};
use tempfile::NamedTempFile;

fn language_template_fixture() -> Vec<String> {
    vec![
        "Rust".to_string(),
        "Python".to_string(),
        "JavaScript".to_string(),
        "Go".to_string(),
    ]
}

/// Tests that `write_list` writes each template on its own line.
#[test]
fn test_template_write_list_outputs_lines() {
    let templates = language_template_fixture();

    let output = FileOutput;
    let mut buffer = Cursor::new(Vec::new());

    output.write_list(templates.clone(), &mut buffer).unwrap();

    let result = String::from_utf8(buffer.into_inner()).unwrap();
    // Expect each language to be on a new line
    let expected = templates.into_iter().map(|s| s + "\n").collect::<String>();

    assert_eq!(result, expected);
}

/// Tests that `write_list` writes nothing for no languages.
#[test]
fn test_template_write_list_outputs_lines_no_languages() {
    let templates = vec![];

    let output = FileOutput;
    let mut buffer = Cursor::new(Vec::new());

    output.write_list(templates.clone(), &mut buffer).unwrap();

    let result = String::from_utf8(buffer.into_inner()).unwrap();
    let expected = "";

    assert_eq!(result, expected);
}

/// Tests that `write_table` writes templates correctly.
#[test]
fn test_template_write_table_outputs_columns() {
    let templates = language_template_fixture();

    let output = FileOutput;
    let mut buffer = Cursor::new(Vec::new());

    output.write_table(templates.clone(), &mut buffer).unwrap();

    let result = String::from_utf8(buffer.into_inner()).unwrap();

    // Testing exact formatting is redundantly testing the external crate,
    // but we can assert that all templates appear in the output.
    for template in templates {
        assert!(result.contains(&template));
    }

    let lines: Vec<_> = result.lines().collect();
    assert!(!lines.is_empty());
}

/// Tests that `write_table` writes nothing for no languages.
#[test]
fn test_template_write_table_outputs_columns_no_languages() {
    let templates = vec![];

    let output = FileOutput;
    let mut buffer = Cursor::new(Vec::new());

    output.write_table(templates.clone(), &mut buffer).unwrap();

    let result = String::from_utf8(buffer.into_inner()).unwrap();

    for template in templates {
        assert!(result.contains(&template));
    }

    let lines: Vec<_> = result.lines().collect();
    assert!(lines.is_empty());
}

fn read_lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn fixture_sample_contents() -> Vec<String> {
    vec!["line one".into(), "line two".into(), "line three".into()]
}

/// Tests that writing to a new file is successful
#[test]
fn test_write_new_file() {
    let tmp_file = NamedTempFile::new().unwrap();
    let path = tmp_file.path();
    let contents = fixture_sample_contents();

    let file_output = FileOutput;
    file_output
        .write_gitignore(contents.clone(), false, path)
        .unwrap();

    let result = read_lines(path);
    assert_eq!(result, contents);
}

/// Tests that writing without append will overwrite existing file
#[test]
fn test_overwrite_an_existing_file() {
    let tmp_file = NamedTempFile::new().unwrap();
    let path = tmp_file.path();

    let first = fixture_sample_contents();
    let second = vec!["start".into()];

    let file_output = FileOutput;
    file_output
        .write_gitignore(first.clone(), false, path)
        .unwrap();
    file_output
        .write_gitignore(second.clone(), false, path)
        .unwrap();

    let result = read_lines(path);
    assert_eq!(result, second);
}

/// Tests that appending to an existing file works as expected
#[test]
fn test_append_to_existing_file() {
    let tmp_file = NamedTempFile::new().unwrap();
    let path = tmp_file.path();

    let first = vec!["start".into()];
    let second = fixture_sample_contents();

    let file_output = FileOutput;
    // First write
    file_output
        .write_gitignore(first.clone(), false, path)
        .unwrap();
    // Then append
    file_output
        .write_gitignore(second.clone(), true, path)
        .unwrap();

    let result = read_lines(path);
    let expected = [first, second].concat();
    assert_eq!(result, expected);
}

/// Tests that appending to an non-existing file fails
#[test]
fn test_append_to_missing_file_should_fail() {
    let path = PathBuf::from("nonexistent_dir/file.txt");

    let file_output = FileOutput;
    let result = file_output.write_gitignore(fixture_sample_contents(), true, &path);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
}
