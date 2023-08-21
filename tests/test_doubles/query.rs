use std::{error::Error, rc::Rc, sync::Arc};

use rs_subito_alert::{
    query_db::{query_api::QueryApi, search::Search},
    scraper::item_result::ItemResult,
};

#[derive(Default)]
pub struct QueryDbSpy {
    pub deletions: Vec<String>,
    pub gets: Vec<String>,
    pub invocations: Vec<Rc<Search>>,
    pub lists: Vec<()>,
}

#[derive(Default)]
pub struct QueryDbFake;

impl QueryDbSpy {
    pub fn new() -> QueryDbSpy {
        QueryDbSpy {
            invocations: vec![],
            deletions: vec![],
            gets: vec![],
            lists: vec![],
        }
    }
}

impl QueryDbFake {
    pub fn new() -> QueryDbFake {
        QueryDbFake {}
    }
}

impl QueryApi for QueryDbSpy {
    fn add_search(&mut self, search: Rc<Search>) -> Result<(), Box<dyn Error>> {
        self.invocations.push(search);
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.deletions.push(name);
        Ok(())
    }

    fn fetch_all_searches(
        &mut self,
    ) -> Result<Vec<Rc<Search>>, Box<(dyn std::error::Error + 'static)>> {
        self.lists.push(());
        Ok(vec![])
    }

    fn fetch_all_items(
        &mut self,
    ) -> Result<Vec<Rc<String>>, Box<(dyn std::error::Error + 'static)>> {
        todo!()
    }

    fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl QueryApi for QueryDbFake {
    fn add_search(&mut self, _: Rc<Search>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn delete_search(&mut self, _: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn fetch_all_searches(
        &mut self,
    ) -> Result<Vec<Rc<Search>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(vec![
            Rc::new(Search::new("Test".to_string(), "test".to_string())),
            Rc::new(Search::new("Test2".to_string(), "test2".to_string())),
            Rc::new(Search::new("Test3".to_string(), "test3".to_string())),
        ])
    }

    fn fetch_all_items(
        &mut self,
    ) -> Result<Vec<Rc<String>>, Box<(dyn std::error::Error + 'static)>> {
        Ok(vec![
            Rc::new(String::from("test")),
            Rc::new(String::from("test2")),
        ])
    }

    fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
