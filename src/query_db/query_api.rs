use std::{error::Error, rc::Rc, sync::Arc};

use crate::scraper::item_result::ItemResult;

use super::search::Search;

pub trait QueryApi {
    fn add_search(&mut self, search: Rc<Search>) -> Result<(), Box<dyn Error>>;
    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>>;
    fn fetch_all_searches(&mut self) -> Result<Vec<Rc<Search>>, Box<dyn Error>>;
    fn fetch_all_items(&mut self) -> Result<Vec<Rc<String>>, Box<dyn Error>>;
    fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>>;
}
