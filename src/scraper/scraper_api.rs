use std::error::Error;

use crate::query_db::search::Search;

use super::item_result::ItemResult;

pub trait ScraperApi {
    fn run_query(&mut self, search: Search) -> Result<Vec<ItemResult>, Box<dyn Error>>;
}
