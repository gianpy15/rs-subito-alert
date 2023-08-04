use std::error::Error;

use crate::scraper::item_result::ItemResult;

use super::{
    db::{DataBase, SerializerApi},
    query::QueryApi,
    search::Search,
};

pub struct QueryEngine<'a, S> {
    database: &'a mut DataBase,
    serializer: &'a mut S,
}

impl<'a, S> QueryEngine<'a, S> {
    pub fn new(database: &'a mut DataBase, serializer: &'a mut S) -> Self {
        Self {
            database,
            serializer,
        }
    }
}

impl<'a, S> QueryApi for QueryEngine<'a, S>
where
    S: SerializerApi,
{
    fn add_search(&mut self, search: Search) -> Result<(), Box<dyn Error>> {
        self.database.add(&search);
        self.serializer.serialize(self.database)?;
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.database.delete(name);
        self.serializer.serialize(self.database)?;
        Ok(())
    }

    fn fetch_all_searches(&mut self) -> Result<Vec<Search>, Box<dyn Error>> {
        Ok(self.database.get_all_searches())
    }

    fn fetch_all_items(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self.database.get_all_items())
    }

    fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        self.database.add_items(items);
        self.serializer.serialize(self.database)?;
        Ok(())
    }
}
