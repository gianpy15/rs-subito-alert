use std::{error::Error, rc::Rc};

use crate::query_db::search::Search;

use super::item_result::ItemResult;

pub trait ScraperApi {
    fn run_query(&mut self, search: Rc<Search>) -> Result<Vec<Rc<ItemResult>>, Box<dyn Error>>;
}
