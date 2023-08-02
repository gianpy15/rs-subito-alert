use std::io::Error;

use super::search::Search;

pub trait QueryApi {
    fn add_search(&mut self, search: Search) -> Result<(), Error>;
}