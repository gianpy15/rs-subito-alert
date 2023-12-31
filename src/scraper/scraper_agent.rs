use crate::query_db::search::Search;
use async_trait::async_trait;
use regex::Regex;
use soup::prelude::*;
use std::{error::Error, sync::Arc};

use super::{
    downloader::download_api::DownloadApi, item_result::ItemResult, scraper_api::ScraperApi,
};

pub struct ScraperAgent<T> {
    download_api: Arc<T>,
}

impl<T> ScraperAgent<T>
where
    T: DownloadApi,
{
    pub fn new(download_api: Arc<T>) -> Self {
        Self { download_api }
    }
}

#[async_trait]
impl<T> ScraperApi for ScraperAgent<T>
where
    T: DownloadApi + Send + Sync,
{
    async fn run_query(&self, search: Arc<Search>) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>> {
        let mut results: Vec<Arc<ItemResult>> = vec![];
        let body = self.download_api.get_content_from(search).await?;

        let soup = Soup::new(&body);

        let product_list_items = soup
            .tag("div")
            .class(Regex::new("item-key-data")?)
            .find_all();

        for product in product_list_items {
            let parent_error = "Cannot get parent";

            let name = product
                .tag("h2")
                .find()
                .ok_or("Cannot collect title")?
                .text();

            let uri = product
                .parent()
                .ok_or(parent_error)?
                .parent()
                .ok_or(parent_error)?
                .parent()
                .ok_or(parent_error)?
                .parent()
                .ok_or(parent_error)?
                .get("href")
                .ok_or("Cannot collect uri")?;

            let price_sections = &product
                .tag("p")
                .class(Regex::new("price")?)
                .find()
                .ok_or("Cannot get price block")?
                .children;
            let borrowed_price_sections = price_sections.borrow();
            let price = borrowed_price_sections
                .get(0)
                .map(|node| node.text())
                .and_then(|mut p| {
                    p.truncate(p.len() - 5);
                    p.parse::<i32>().ok()
                });

            let town = product
                .tag("span")
                .class(Regex::new("town")?)
                .find()
                .map(|node| node.text());
            let city = product
                .tag("span")
                .class(Regex::new("city")?)
                .find()
                .map(|node| node.text());

            let date = product
                .tag("span")
                .class(Regex::new("date")?)
                .find()
                .map(|node| node.text());

            let state = borrowed_price_sections.get(1).map(|node| node.text());

            let result = Arc::new(ItemResult::new(name, uri, date, price, town, city, state));
            results.push(result);
        }
        Ok(results)
    }
}
