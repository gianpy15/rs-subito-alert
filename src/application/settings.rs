use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    scraping_interval: i32,
}

impl Settings {
    pub fn new(scraping_interval: i32) -> Self {
        Self { scraping_interval }
    }

    pub fn set_scraping_interval(&mut self, scraping_interval: i32) {
        self.scraping_interval = scraping_interval;
    }

    pub fn get_scraping_interval(&self) -> i32 {
        self.scraping_interval
    }
}
