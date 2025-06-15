use httpmock::Method::GET;
use httpmock::MockServer;
use reqwest::Client;
use tokio;

use blindfold::client::{GitIgnoreClient, GitIgnoreIOClient};

/// Test that list templates works as expected
#[tokio::test]
async fn test_list_templates() {
    let mock_server = MockServer::start();
    let languages = String::from("Rust,Python,JavaScript\nGo,Java,Kotlin");

    let list_mock = mock_server.mock(|when, then| {
        when.method(GET).path("/list");
        then.status(200)
            .header("content-type", "text/plain")
            .body(&languages);
    });

    let client = GitIgnoreIOClient::new(mock_server.url("/").as_str(), Client::new());
    let templates = client.list_templates().await;

    list_mock.assert();

    assert_eq!(
        templates.unwrap(),
        vec!["Rust", "Python", "JavaScript", "Go", "Java", "Kotlin"]
    );
}

/// Test that fetching gitignore contents works as expected
#[tokio::test]
async fn test_fetch_gitignore_contents() {
    let mock_server = MockServer::start();

    // Example languages param
    let languages = vec!["Rust", "Python", "JavaScript"];
    let expected_path = format!("{}{}", "", languages.join(","));

    // Prepare the mocked response body, simulating the gitignore content lines
    let gitignore_body = "\
        # Rust gitignore content line 1
        target/
        Cargo.lock

        # Python gitignore content line 1
        __pycache__/
        *.pyc

        # JavaScript gitignore content line 1
        node_modules/
        dist/
    ";

    // Mock the expected GET request to URL like "{base_url}Rust,Python,JavaScript"
    let fetch_mock = mock_server.mock(|when, then| {
        when.method("GET").path(format!("/{}", expected_path));
        then.status(200)
            .header("content-type", "text/plain")
            .body(gitignore_body);
    });

    let client = GitIgnoreIOClient::new(&mock_server.url("/").as_str(), Client::new());
    let result = client.fetch_gitinore_contents(&languages).await;

    fetch_mock.assert();

    let expected_lines: Vec<String> = gitignore_body
        .lines()
        .map(|line| line.to_string())
        .collect();

    assert_eq!(result.unwrap(), expected_lines);
}

#[tokio::test]
async fn test_fetch_gitignore_contents_invalid_language() {
    let mock_server = MockServer::start();

    let languages = vec!["Cobra"];
    let expected_path = format!("/{}", languages.join(","));
    let error_body = "error message";

    let fetch_mock = mock_server.mock(|when, then| {
        when.method("GET").path(expected_path.clone());
        then.status(404)
            .header("content-type", "text/plain")
            .body(error_body);
    });

    let client = GitIgnoreIOClient::new(&mock_server.url("/").as_str(), Client::new());
    let result = client.fetch_gitinore_contents(&languages).await;

    fetch_mock.assert();

    assert!(
        result.is_none(),
        "Expected None for 404 response with error body"
    );
}
