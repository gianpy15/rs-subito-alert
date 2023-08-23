use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait NotificationApi {
    async fn notify(&mut self, item: String) -> Result<(), Box<dyn Error>>; //TODO: Item should beocme a ItemResult
}
