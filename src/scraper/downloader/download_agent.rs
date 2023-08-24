use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::query_db::search::Search;

use super::download_api::DownloadApi;

#[derive(Clone)]
pub struct DownloadAgent {
    base_uri: String,
}

impl DownloadAgent {
    pub fn new(base_uri: String) -> Self {
        Self { base_uri }
    }
}

impl Default for DownloadAgent {
    fn default() -> Self {
        Self::new("https://www.subito.it/annunci-italia/vendita/usato/?q=".to_string())
    }
}

#[async_trait]
impl DownloadApi for DownloadAgent {
    async fn get_content_from(&self, search: Arc<Search>) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(self.get_search_uri(search)).await?;
        let body = response.text().await?;
        Ok(body)
    }

    fn get_search_uri(&self, search: Arc<Search>) -> String {
        let query = search.query.replace(' ', "%20");
        self.get_base_uri() + &query
    }

    fn get_base_uri(&self) -> String {
        self.base_uri.clone()
    }
}
