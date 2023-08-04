use std::{fs, path::Path, rc::Rc};

use rs_subito_alert::{
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

impl Default for DownloadFake {
    fn default() -> Self {
        Self::new()
    }
}

impl DownloadApi for DownloadFake {
    fn get_content_from(&self, _: Rc<Search>) -> Result<String, Box<dyn std::error::Error>> {
        let uri = self.get_base_uri();
        let path = Path::new(&uri);
        let html = fs::read_to_string(path)?;
        Ok(html)
    }

    fn get_search_uri(&self, _: Rc<Search>) -> String {
        "tests/resources/example_page.html".to_string()
    }

    fn get_base_uri(&self) -> String {
        "tests/resources/example_page.html".to_string()
    }
}

impl ScraperSpy {
    pub fn new() -> ScraperSpy {
        ScraperSpy { invocations: 0 }
    }
}

impl ScraperApi for ScraperSpy {
    fn run_query(&mut self, search: Rc<Search>) -> Result<Vec<Rc<ItemResult>>, Box<(dyn std::error::Error + 'static)>> {
        self.invocations += 1;
        Ok(vec![Rc::new(ItemResult::default(&search.name, &search.query))])
    }
}

impl ScraperApi for ScraperFake {
    fn run_query(&mut self, search: Rc<Search>) -> Result<Vec<Rc<ItemResult>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(vec![Rc::new(ItemResult::default(&search.name, &search.query))])
    }
}
