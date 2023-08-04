use std::{error::Error, rc::Rc};

use crate::query_db::search::Search;
use isahc::ReadResponseExt;

pub trait DownloadApi {
    fn get_content_from(&self, search: Rc<Search>) -> Result<String, Box<dyn Error>>;
    fn get_search_uri(&self, search: Rc<Search>) -> String;
    fn get_base_uri(&self) -> String;
}

pub struct DownloadAgent {
    base_uri: String,
}

impl DownloadAgent {
    pub fn new(base_uri: String) -> Self {
        Self { base_uri }
    }
}

impl Default for DownloadAgent {
    fn default() -> Self {
        Self::new("https://www.subito.it/annunci-italia/vendita/usato/?q=".to_string())
    }
}

impl DownloadApi for DownloadAgent {
    fn get_content_from(&self, search: Rc<Search>) -> Result<String, Box<dyn Error>> {
        let mut response = isahc::get(self.get_search_uri(search))?;
        let body = response.text()?;
        Ok(body)
    }

    fn get_search_uri(&self, search: Rc<Search>) -> String {
        let query = search.query.replace(' ', "%20");
        self.get_base_uri() + &query
    }

    fn get_base_uri(&self) -> String {
        self.base_uri.clone()
    }
}
