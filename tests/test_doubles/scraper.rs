use async_trait::async_trait;
use rs_subito_alert::{
    query_db::search::Search,
    scraper::{
        downloader::download_api::DownloadApi, item_result::ItemResult, scraper_api::ScraperApi,
    },
};
use std::{path::Path, sync::Arc};
use tokio::{fs, sync::Mutex};

#[derive(Default)]
pub struct ScraperSpy {
    pub invocations: Mutex<i32>,
}

#[derive(Default)]
pub struct ScraperDouble {
    results: Vec<Arc<ItemResult>>,
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

#[async_trait]
impl DownloadApi for DownloadFake {
    async fn get_content_from(&self, _: Arc<Search>) -> Result<String, Box<dyn std::error::Error>> {
        let uri = self.get_base_uri();
        let path = Path::new(&uri);
        let html = fs::read_to_string(path).await?;
        Ok(html)
    }

    fn get_search_uri(&self, _: Arc<Search>) -> String {
        "tests/resources/example_page".to_string()
    }

    fn get_base_uri(&self) -> String {
        "tests/resources/example_page".to_string()
    }
}

impl ScraperSpy {
    pub fn new() -> ScraperSpy {
        ScraperSpy {
            invocations: Mutex::new(0),
        }
    }
}

#[async_trait]
impl ScraperApi for ScraperSpy {
    async fn run_query(
        &self,
        search: Arc<Search>,
    ) -> Result<Vec<Arc<ItemResult>>, Box<(dyn std::error::Error + 'static)>> {
        *self.invocations.lock().await += 1;
        Ok(vec![Arc::new(ItemResult::default(
            &search.name,
            &search.query,
        ))])
    }
}

#[async_trait]
impl ScraperApi for ScraperFake {
    async fn run_query(
        &self,
        search: Arc<Search>,
    ) -> Result<Vec<Arc<ItemResult>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(vec![Arc::new(ItemResult::default(
            &search.name,
            &search.query,
        ))])
    }
}

impl ScraperDouble {
    pub fn new() -> Self {
        Self { results: vec![] }
    }
    pub fn set_results(&mut self, results: Vec<ItemResult>) {
        for result in results {
            self.results.push(Arc::new(result));
        }
    }
}

#[async_trait]
impl ScraperApi for ScraperDouble {
    async fn run_query(
        &self,
        search: Arc<Search>,
    ) -> Result<Vec<Arc<ItemResult>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(self
            .results
            .iter()
            .map(|r| r.clone())
            .filter(|r| r.get_uri() == search.query)
            .collect())
    }
}
