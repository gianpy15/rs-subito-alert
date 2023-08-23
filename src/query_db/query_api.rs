use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::scraper::item_result::ItemResult;

use super::search::Search;

#[async_trait]
pub trait QueryApi {
    async fn add_search(&mut self, search: Arc<Search>) -> Result<(), Box<dyn Error>>;
    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>>;
    fn fetch_all_searches(&mut self) -> Result<Vec<Arc<Search>>, Box<dyn Error>>;
    fn fetch_all_items(&mut self) -> Result<Vec<Arc<String>>, Box<dyn Error>>;
    async fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>>;
}
