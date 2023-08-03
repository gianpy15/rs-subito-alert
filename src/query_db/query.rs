use std::error::Error;

use super::search::Search;

pub trait QueryApi {
    fn add_search(&mut self, search: Search) -> Result<(), Box<dyn Error>>;
    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>>;
    fn fetch_all(&mut self) -> Result<Vec<Search>, Box<dyn Error>>;
}
