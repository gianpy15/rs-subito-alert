use std::{error::Error, rc::Rc};

use crate::query_db::search::Search;
use isahc::ReadResponseExt;

pub trait DownloadApi {
    fn get_content_from(&self, search: Rc<Search>) -> Result<String, Box<dyn Error>>;
    fn get_search_uri(&self, search: Rc<Search>) -> String;
    fn get_base_uri(&self) -> String;
}
