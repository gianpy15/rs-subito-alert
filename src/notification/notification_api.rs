use std::error::Error;

use async_trait::async_trait;

use crate::scraper::item_result::ItemResult;

#[async_trait]
pub trait NotificationApi {
    async fn notify(&self, item: &ItemResult) -> Result<(), Box<dyn Error>>; //TODO: Item should beocme a ItemResult
    async fn add_user(&self, id: &str) -> Result<(), Box<dyn Error>>;
    async fn reset(&self) -> Result<(), Box<dyn Error>>;
}
