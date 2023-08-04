use std::error::Error;

use crate::scraper::item_result::ItemResult;

pub trait NotificationApi {
    fn notify(&mut self, item: &ItemResult) -> Result<(), Box<dyn Error>>;
}
