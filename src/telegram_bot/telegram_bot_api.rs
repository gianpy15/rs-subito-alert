use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use teloxide::Bot;

use crate::types::Application;
#[async_trait]
pub trait TelegramBotApi {
    async fn start(&self, application: Application);
    async fn add_api_key(&self, api_key: &str) -> Result<(), Box<dyn Error>>;
    fn get_bot(&self) -> Arc<Bot>;
}
