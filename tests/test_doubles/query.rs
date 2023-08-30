use std::{error::Error, rc::Rc, sync::Arc};

use async_trait::async_trait;
use rs_subito_alert::{
    query_db::{query_api::QueryApi, search::Search},
    scraper::item_result::ItemResult,
};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct QueryDbDouble {
    pub deletions: Vec<String>,
    pub gets: Vec<String>,
    pub invocations: Vec<Arc<Search>>,
    pub lists: Mutex<i32>,
    pub adds: Vec<Arc<str>>,
    searches: Vec<Arc<Search>>,
    items: Vec<Arc<str>>,
}

impl QueryDbDouble {
    pub fn new() -> QueryDbDouble {
        QueryDbDouble {
            invocations: vec![],
            deletions: vec![],
            gets: vec![],
            adds: vec![],
            lists: Mutex::new(0),
            searches: vec![],
            items: vec![],
        }
    }

    pub fn set_searches(&mut self, searches: Vec<Arc<Search>>) {
        self.searches = searches;
    }

    pub fn set_items(&mut self, items: Vec<Arc<str>>) {
        self.items = items;
    }
}

#[async_trait]
impl QueryApi for QueryDbDouble {
    async fn add_search(&mut self, search: Arc<Search>) -> Result<(), Box<dyn Error>> {
        self.invocations.push(search);
        Ok(())
    }

    async fn delete_search(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.deletions.push(name.to_string());
        Ok(())
    }

    async fn fetch_all_searches(
        &self,
    ) -> Result<Vec<Arc<Search>>, Box<(dyn std::error::Error + 'static)>> {
        *self.lists.lock().await += 1;
        Ok(self.searches.clone())
    }

    async fn fetch_all_items(&self) -> Result<Vec<Arc<str>>, Box<(dyn Error + 'static)>> {
        Ok(self.items.clone())
    }

    async fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        for item in items {
            self.adds.push(Arc::clone(&item.get_uri()));
        }
        Ok(())
    }
}
