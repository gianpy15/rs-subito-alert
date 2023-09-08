use std::error::Error;

use async_trait::async_trait;
use rs_subito_alert::{
    notification::notification_api::NotificationApi, scraper::item_result::ItemResult,
};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct NotifierSpy {
    pub invocations: Mutex<Vec<ItemResult>>,
    pub users: Mutex<Vec<String>>,
}

impl NotifierSpy {
    pub fn new() -> Self {
        Self {
            invocations: Mutex::new(vec![]),
            users: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl NotificationApi for NotifierSpy {
    async fn notify(&self, result: &ItemResult) -> Result<(), Box<dyn Error>> {
        self.invocations.lock().await.push(result.clone());
        Ok(())
    }

    async fn add_user(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.users.lock().await.push(String::from(id));
        Ok(())
    }

    async fn reset(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
