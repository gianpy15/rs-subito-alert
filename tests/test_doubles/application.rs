use std::{error::Error, rc::Rc};

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

impl ApplicationApi for ApplicationDouble {
    fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.invocations.push(Some((name, query)));
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn list(&mut self) -> Result<Vec<Rc<Search>>, Box<dyn Error>> {
        self.invocations.push(None);
        Ok(vec![])
    }

    fn scrape(&mut self) -> Result<Vec<Rc<ItemResult>>, Box<dyn Error>> {
        todo!()
    }
}
