use crate::client::GitIgnoreClient;

pub struct GitIgnoreService<Client: GitIgnoreClient> {
    client: Client,
}

impl<Client: GitIgnoreClient> GitIgnoreService<Client> {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list_templates(&self) -> Option<Vec<String>> {
        let available_templates = self.client.list_templates().await?;
        Some(available_templates)
    }

    pub async fn get_gitignore_contents(&self, languages: &[&str]) -> Option<Vec<String>> {
        let gitignore_contents = self.client.fetch_gitinore_contents(&languages).await?;
        Some(gitignore_contents)
    }
}
