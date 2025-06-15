use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GitIgnoreClient {
    async fn list_templates(&self) -> Option<Vec<String>>;
    async fn fetch_gitinore_contents(&self, languages: &[&str]) -> Option<Vec<String>>;
}

pub struct GitIgnoreIOClient {
    base_url: String,
    client: Client,
}

impl GitIgnoreIOClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl GitIgnoreClient for GitIgnoreIOClient {
    /// Lists all gitignore templates from `GitIgnore.io`.
    ///
    /// Sends a GET request to the `<base_url>/list` endpoint,
    /// parses the response into a list of template names.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the response
    /// cannot be parsed.
    async fn list_templates(&self) -> Option<Vec<String>> {
        let url = format!("{}list", self.base_url);
        let response = self.client.get(&url).send().await.ok()?.text().await.ok()?;
        let templates = response
            .lines()
            .flat_map(|line| line.split(',').map(|s| s.trim().to_string()))
            .collect::<Vec<String>>();

        Some(templates)
    }

    /// Fetches `.gitignore` templates for the specified languages.
    ///
    /// # Arguments
    ///
    /// * `languages` - The requested languages.
    ///
    /// Sends a request to the GitIgnore.io to retrieve templates.
    /// Parses the response into a vector of template contents.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the response
    /// cannot be parsed.
    async fn fetch_gitinore_contents(&self, languages: &[&str]) -> Option<Vec<String>> {
        let url = format!("{}{}", self.base_url, languages.join(","));
        let response = self.client.get(&url).send().await.ok()?.text().await.ok()?;

        let templates = response
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        Some(templates)
    }
}
