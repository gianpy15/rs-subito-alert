use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    scraping_timeout: i32,
}

impl Settings {
    pub fn new(scraping_timeout: i32) -> Self {
        Self { scraping_timeout }
    }

    pub fn set_scraping_timeout(&mut self, scraping_timeout: i32) {
        self.scraping_timeout = scraping_timeout;
    }

    pub fn get_scraping_timeout(&self) -> i32 {
        self.scraping_timeout.clone()
    }
}
