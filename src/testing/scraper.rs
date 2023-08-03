use std::{fs, path::Path};

use crate::{
    query_db::search::Search,
    scraper::{download_api::DownloadApi, item_result::ItemResult, scraper_api::ScraperApi},
};

#[derive(Default)]
pub struct ScraperSpy {
    pub invocations: i32,
}

#[derive(Default)]
pub struct ScraperFake;

pub struct DownloadFake;

impl DownloadFake {
    pub fn new() -> Self {
        Self {}
    }
}

impl DownloadApi for DownloadFake {
    fn get_content_from(&self, _: Search) -> Result<String, Box<dyn std::error::Error>> {
        let uri = self.get_base_uri();
        let path = Path::new(&uri);
        let html = fs::read_to_string(path)?;
        Ok(html)
    }

    fn get_search_uri(&self, _: Search) -> String {
        "resources/example_page.html".to_string()
    }

    fn get_base_uri(&self) -> String {
        "resources/example_page.html".to_string()
    }
}

impl ScraperSpy {
    pub fn new() -> ScraperSpy {
        ScraperSpy { invocations: 0 }
    }
}

impl ScraperApi for ScraperSpy {
    fn run_query(&mut self, search: Search) -> Result<Vec<ItemResult>, Box<dyn std::error::Error>> {
        self.invocations += 1;
        Ok(vec![ItemResult::default(search.name, search.query)])
    }
}

impl ScraperApi for ScraperFake {
    fn run_query(&mut self, search: Search) -> Result<Vec<ItemResult>, Box<dyn std::error::Error>> {
        Ok(vec![ItemResult::default(search.name, search.query)])
    }
}
