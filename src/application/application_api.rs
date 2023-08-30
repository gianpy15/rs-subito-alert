use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::{query_db::search::Search, scraper::item_result::ItemResult};

#[async_trait]
pub trait ApplicationApi {
    async fn add_search(&mut self, name: &str, query: &str) -> Result<(), Box<dyn Error>>;
    async fn delete_search(&mut self, name: &str) -> Result<(), Box<dyn Error>>;
    async fn list(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>>;
    async fn scrape(&self, notify: Option<bool>) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>>;
    async fn add_user(&self, id: String) -> Result<(), Box<dyn Error>>;
}
