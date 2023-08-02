use std::io::Error;

use crate::query_db::{query::QueryApi, search::Search};

pub struct QueryDbSpy {
    pub deletions: Vec<String>,
    pub gets: Vec<String>,
    pub invocations: Vec<Search>,
    pub lists: Vec<()>,
}

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

impl QueryApi for QueryDbSpy {
    fn add_search(&mut self, search: Search) -> Result<(), Error> {
        self.invocations.push(search);
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Error> {
        self.deletions.push(name);
        Ok(())
    }

    fn get_search(&mut self, name: String) -> Option<Search> {
        self.gets.push(name);
        None
    }

    fn fetch_all(&mut self) -> Result<Vec<Search>, Error> {
        self.lists.push(());
        Ok(vec![])
    }
}
