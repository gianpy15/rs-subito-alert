use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait UserInterfaceApi {
    async fn start_cli(&self);
    async fn add_api_key(&self, api_key: &str) -> Result<(), Box<dyn Error>>;
    async fn start_application(&self) -> Result<(), Box<dyn Error>>;
    async fn reset_application(&self) -> Result<(), Box<dyn Error>>;
    fn quit(&self) -> !;
}
