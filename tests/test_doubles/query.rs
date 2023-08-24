use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use rs_subito_alert::{
    query_db::{query_api::QueryApi, search::Search},
    scraper::item_result::ItemResult,
};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct QueryDbSpy {
    pub deletions: Vec<String>,
    pub gets: Vec<String>,
    pub invocations: Vec<Arc<Search>>,
    pub lists: Mutex<i32>,
}

#[derive(Default)]
pub struct QueryDbFake;

impl QueryDbSpy {
    pub fn new() -> QueryDbSpy {
        QueryDbSpy {
            invocations: vec![],
            deletions: vec![],
            gets: vec![],
            lists: Mutex::new(0),
        }
    }
}

impl QueryDbFake {
    pub fn new() -> QueryDbFake {
        QueryDbFake {}
    }
}

#[async_trait]
impl QueryApi for QueryDbSpy {
    async fn add_search(&mut self, search: Arc<Search>) -> Result<(), Box<dyn Error>> {
        self.invocations.push(search);
        Ok(())
    }

    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.deletions.push(name);
        Ok(())
    }

    async fn fetch_all_searches(
        &self,
    ) -> Result<Vec<Arc<Search>>, Box<(dyn std::error::Error + 'static)>> {
        *self.lists.lock().await += 1;
        Ok(vec![])
    }

    async fn fetch_all_items(
        &self,
    ) -> Result<Vec<Arc<String>>, Box<(dyn std::error::Error + 'static)>> {
        todo!()
    }

    async fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[async_trait]
impl QueryApi for QueryDbFake {
    async fn add_search(&mut self, _: Arc<Search>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete_search(&mut self, _: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn fetch_all_searches(
        &self,
    ) -> Result<Vec<Arc<Search>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(vec![
            Arc::new(Search::new("Test".to_string(), "test".to_string())),
            Arc::new(Search::new("Test2".to_string(), "test2".to_string())),
            Arc::new(Search::new("Test3".to_string(), "test3".to_string())),
        ])
    }

    async fn fetch_all_items(
        &self,
    ) -> Result<Vec<Arc<String>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(vec![
            Arc::new(String::from("test")),
            Arc::new(String::from("test2")),
        ])
    }

    async fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
