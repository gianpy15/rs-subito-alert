use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::{query_db::search::Search, scraper::item_result::ItemResult};

#[async_trait]
pub trait ApplicationApi {
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>>;
    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>>;
    async fn list(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>>;
    async fn scrape(&self) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>>;
}
