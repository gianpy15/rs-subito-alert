use std::io::Error;

use crate::query_db::search::Search;

pub trait Application {
    fn add_search(&mut self, name: String, query: String) -> Result<(), Error>;
    fn delete_search(&mut self, name: String) -> Result<(), Error>;
    fn list(&mut self) -> Result<Vec<Search>, Error>;
    fn scrape(&mut self) -> Result<(), Error>;
}
