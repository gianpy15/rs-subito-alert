use std::io::Error;

use super::search::Search;

pub trait QueryApi {
    fn add_search(&mut self, search: Search) -> Result<(), Error>;
    fn delete_search(&mut self, name: String) -> Result<(), Error>;
    fn get_search(&mut self, name: String) -> Option<Search>;
    fn fetch_all(&mut self) -> Result<Vec<Search>, Error>;
}