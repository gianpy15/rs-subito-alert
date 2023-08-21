use crate::scraper::item_result::ItemResult;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc, sync::Arc,
};

use super::search::Search;

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataBase {
    searches: HashMap<Rc<String>, Rc<Search>>,
    items: HashSet<Rc<String>>,
}

impl DataBase {
    pub fn new(searches: Vec<Rc<Search>>, items: Vec<Rc<String>>) -> Self {
        let mut search_db = HashMap::new();
        let mut items_db = HashSet::new();
        searches.iter().for_each(|search| {
            search_db.insert(Rc::clone(&search.name), Rc::clone(search));
        });
        items.iter().for_each(|item| {
            items_db.insert(item.clone());
        });
        Self {
            searches: search_db,
            items: items_db,
        }
    }

    pub fn add(&mut self, search: Rc<Search>) {
        self.searches.insert(Rc::clone(&search.name), search);
    }

    pub fn add_items(&mut self, search: Vec<ItemResult>) {
        search.iter().for_each(|item| {
            self.items.insert(item.get_uri());
        });
    }

    pub fn delete(&mut self, name: String) {
        self.searches.remove(&name);
    }

    pub fn get_all_searches(&mut self) -> Vec<Rc<Search>> {
        self.searches.values().cloned().collect()
    }
    pub fn get_all_items(&mut self) -> Vec<Rc<String>> {
        self.items.iter().cloned().collect()
    }
}
