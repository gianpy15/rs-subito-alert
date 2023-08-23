use std::{error::Error, sync::Arc};

use crate::query_db::search::Search;
use async_trait::async_trait;

#[async_trait]
pub trait DownloadApi {
    async fn get_content_from(&self, search: Arc<Search>) -> Result<String, Box<dyn Error>>;
    fn get_search_uri(&self, search: Arc<Search>) -> String;
    fn get_base_uri(&self) -> String;
}
