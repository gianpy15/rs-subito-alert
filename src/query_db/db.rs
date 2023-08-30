use crate::scraper::item_result::ItemResult;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use super::search::Search;

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataBase {
    searches: HashMap<Arc<str>, Arc<Search>>,
    items: HashSet<Arc<str>>,
}

impl DataBase {
    pub fn new(searches: Vec<Arc<Search>>, items: Vec<Arc<str>>) -> Self {
        let mut search_db = HashMap::new();
        let mut items_db = HashSet::new();
        searches.iter().for_each(|search| {
            search_db.insert(Arc::clone(&search.name), Arc::clone(search));
        });
        items.iter().for_each(|item| {
            items_db.insert(item.clone());
        });
        Self {
            searches: search_db,
            items: items_db,
        }
    }

    pub fn add(&mut self, search: Arc<Search>) {
        self.searches.insert(Arc::clone(&search.name), search);
    }

    pub fn add_items(&mut self, search: Vec<ItemResult>) {
        search.iter().for_each(|item| {
            self.items.insert(item.get_uri());
        });
    }

    pub fn delete(&mut self, name: &str) {
        self.searches.remove(name);
    }

    pub fn get_all_searches(&self) -> Vec<Arc<Search>> {
        self.searches.values().cloned().collect()
    }
    pub fn get_all_items(&self) -> Vec<Arc<str>> {
        self.items.iter().cloned().collect()
    }
}
