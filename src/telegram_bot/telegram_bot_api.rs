use std::error::Error;

use async_trait::async_trait;
#[async_trait]
pub trait TelegramBotApi {
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>>;
    async fn list_searches(&mut self) -> Result<(), Box<dyn Error>>;
}
