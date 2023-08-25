use std::error::Error;

use async_trait::async_trait;
use rs_subito_alert::notification::notification_api::NotificationApi;
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
    async fn notify(&self, _: String) -> Result<(), Box<dyn Error>> {
        *self.invocations.lock().await += 1;
        Ok(())
    }

    async fn add_user(&self, id: String) -> Result<(), Box<dyn Error>> {
        self.users.lock().await.push(id);
        Ok(())
    }
}
