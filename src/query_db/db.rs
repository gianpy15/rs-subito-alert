use crate::scraper::item_result::ItemResult;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use super::search::Search;

pub trait SerializerApi {
    fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn Error>>;
}

pub trait DataBaseApi {}

pub struct Serializer {}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct DataBase {
    searches: HashMap<String, Search>,
    items: HashSet<String>,
}

impl DataBase {
    pub fn new(searches: Vec<Search>, items: Vec<String>) -> Self {
        let mut search_db = HashMap::new();
        let mut items_db = HashSet::new();
        searches.iter().for_each(|search| {
            search_db.insert(search.name.clone(), search.clone());
        });
        items.iter().for_each(|item| {
            items_db.insert(item.clone());
        });
        Self {
            searches: search_db,
            items: items_db,
        }
    }

    pub fn add(&mut self, search: &Search) {
        self.searches.insert(search.name.clone(), search.clone());
    }

    pub fn add_items(&mut self, search: Vec<ItemResult>) {
        search.iter().for_each(|item| {
            self.items.insert(item.get_uri());
        });
    }

    pub fn delete(&mut self, name: String) {
        self.searches.remove(&name);
    }

    pub fn get_all_searches(&mut self) -> Vec<Search> {
        Vec::from_iter(self.searches.values().cloned())
    }
    pub fn get_all_items(&mut self) -> Vec<String> {
        Vec::from_iter(self.items.iter().cloned())
    }
}
