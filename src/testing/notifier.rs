use crate::notification::notification_api::NotificationApi;

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
    fn notify(
        &mut self,
        _: &crate::scraper::item_result::ItemResult,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.invocations += 1;
        Ok(())
    }
}
