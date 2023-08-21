use std::{error::Error, rc::Rc, sync::Arc};

use crate::{query_db::search::Search, scraper::item_result::ItemResult};

pub trait ApplicationApi {
    fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>>;
    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>>;
    fn list(&mut self) -> Result<Vec<Rc<Search>>, Box<dyn Error>>;
    fn scrape(&mut self) -> Result<Vec<Rc<ItemResult>>, Box<dyn Error>>;
}
