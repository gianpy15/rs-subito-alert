use rs_subito_alert::{
    notification::notification_api::NotificationApi, scraper::item_result::ItemResult,
};

#[derive(Default)]
pub struct NotifierSpy {
    pub invocations: i32,
}

impl NotifierSpy {
    pub fn new(invocations: i32) -> Self {
        Self { invocations }
    }
}

impl NotificationApi for NotifierSpy {
    fn notify(&mut self, _: &ItemResult) -> Result<(), Box<dyn std::error::Error>> {
        self.invocations += 1;
        Ok(())
    }
}
