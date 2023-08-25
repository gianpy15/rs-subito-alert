use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use rs_subito_alert::{
    application::application_api::ApplicationApi, query_db::search::Search,
    scraper::item_result::ItemResult,
};
use tokio::sync::Mutex;

pub struct ApplicationDouble {
    pub invocations: Mutex<Vec<Option<(String, String)>>>,
}

impl ApplicationDouble {
    pub fn new() -> Self {
        Self {
            invocations: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl ApplicationApi for ApplicationDouble {
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.invocations.lock().await.push(Some((name, query)));
        Ok(())
    }

    async fn delete_search(&mut self, _name: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>> {
        self.invocations.lock().await.push(None);
        Ok(vec![])
    }

    async fn scrape(&self) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>> {
        todo!()
    }

    async fn add_user(&self, _id: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
