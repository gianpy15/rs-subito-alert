use std::error::Error;

use async_trait::async_trait;
use rs_subito_alert::notification::notification_api::NotificationApi;

#[derive(Default)]
pub struct NotifierSpy {
    pub invocations: i32,
}

impl NotifierSpy {
    pub fn new(invocations: i32) -> Self {
        Self { invocations }
    }
}

#[async_trait]
impl NotificationApi for NotifierSpy {
    async fn notify(&mut self, _: String) -> Result<(), Box<dyn Error>> {
        self.invocations += 1;
        Ok(())
    }
}
