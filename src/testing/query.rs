use std::error::Error;

use crate::query_db::{query::QueryApi, search::Search};

#[derive(Default)]
pub struct QueryDbSpy {
    pub deletions: Vec<String>,
    pub gets: Vec<String>,
    pub invocations: Vec<Search>,
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
    fn add_search(&mut self, search: Search) -> Result<(), Box<dyn Error>> {
        self.invocations.push(search);
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.deletions.push(name);
        Ok(())
    }

    fn get_search(&mut self, name: String) -> Option<Search> {
        self.gets.push(name);
        None
    }

    fn fetch_all(&mut self) -> Result<Vec<Search>, Box<dyn Error>> {
        self.lists.push(());
        Ok(vec![])
    }
}

impl QueryApi for QueryDbFake {
    fn add_search(&mut self, _: Search) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn delete_search(&mut self, _: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_search(&mut self, _: String) -> Option<Search> {
        None
    }

    fn fetch_all(&mut self) -> Result<Vec<Search>, Box<dyn Error>> {
        Ok(vec![])
    }
}
