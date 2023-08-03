use std::{collections::HashMap, error::Error};

use super::search::Search;

pub trait SerializerApi {
    fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn Error>>;
}

pub trait DataBaseApi {}

pub struct Serializer {}

#[derive(PartialEq, Debug, Clone)]
pub struct DataBase {
    searches: HashMap<String, Search>,
}

impl DataBase {
    pub fn new(searches: Vec<Search>) -> Self {
        let mut search_db = HashMap::new();
        searches.iter().for_each(|search| {
            search_db.insert(search.name.clone(), search.clone());
        });
        Self {
            searches: search_db,
        }
    }

    pub fn add_search(&mut self, search: &Search) {
        self.searches.insert(search.name.clone(), search.clone());
    }
}

impl Default for DataBase {
    fn default() -> Self {
        Self {
            searches: HashMap::new(),
        }
    }
}
