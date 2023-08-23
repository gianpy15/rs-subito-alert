use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::query_db::search::Search;

use super::item_result::ItemResult;

#[async_trait]
pub trait ScraperApi {
    async fn run_query(
        &mut self,
        search: Arc<Search>,
    ) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>>;
}
