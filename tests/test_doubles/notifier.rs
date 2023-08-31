use std::error::Error;

use async_trait::async_trait;
use rs_subito_alert::{
    notification::notification_api::NotificationApi, scraper::item_result::ItemResult,
};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct NotifierSpy {
    pub invocations: Mutex<i32>,
    pub users: Mutex<Vec<String>>,
}

impl NotifierSpy {
    pub fn new(invocations: Mutex<i32>) -> Self {
        Self {
            invocations,
            users: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl NotificationApi for NotifierSpy {
    async fn notify(&self, _: &ItemResult) -> Result<(), Box<dyn Error>> {
        *self.invocations.lock().await += 1;
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
