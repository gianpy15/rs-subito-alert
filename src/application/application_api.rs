use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::{query_db::search::Search, scraper::item_result::ItemResult};

#[async_trait]
pub trait ApplicationApi {
    async fn add_search(
        &mut self,
        name: &str,
        query: &str,
        max_price: Option<i32>,
    ) -> Result<(), Box<dyn Error>>;
    async fn delete_search(&mut self, name: &str) -> Result<(), Box<dyn Error>>;
    async fn list(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>>;
    async fn scrape(&self, notify: Option<bool>) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>>;
    async fn add_user(&self, id: &str) -> Result<(), Box<dyn Error>>;
    async fn reset(&self) -> Result<(), Box<dyn Error>>;
    async fn set_scraping_interval(&self, interval: i32) -> Result<(), Box<dyn Error>>;
    async fn get_scraping_interval(&self) -> Result<i32, Box<dyn Error>>;
}
