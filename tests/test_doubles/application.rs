use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use rs_subito_alert::{
    application::application_api::ApplicationApi, query_db::search::Search,
    scraper::item_result::ItemResult,
};

pub struct ApplicationDouble {
    pub invocations: Vec<Option<(String, String)>>,
}

impl ApplicationDouble {
    pub fn new() -> Self {
        Self {
            invocations: vec![],
        }
    }
}

#[async_trait]
impl ApplicationApi for ApplicationDouble {
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.invocations.push(Some((name, query)));
        Ok(())
    }

    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn list(&mut self) -> Result<Vec<Arc<Search>>, Box<dyn Error>> {
        self.invocations.push(None);
        Ok(vec![])
    }

    async fn scrape(&mut self) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>> {
        todo!()
    }
}
