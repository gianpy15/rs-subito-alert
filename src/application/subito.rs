use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    notification::notification_api::NotificationApi,
    query_db::{query_api::{QueryApi, self}, search::Search},
    scraper::{item_result::ItemResult, scraper_api::ScraperApi},
};

use super::application_api::ApplicationApi;

pub struct Subito<Q, S, N> {
    query_api: Arc<Mutex<Q>>,
    scraper_api: Arc<S>,
    notification_api: Arc<N>,
}

impl<Q, S, N> Subito<Q, S, N> {
    pub fn new(
        query_api: Arc<Mutex<Q>>,
        scraper_api: Arc<S>,
        notification_api: Arc<N>,
    ) -> Subito<Q, S, N> {
        Subito {
            query_api,
            scraper_api,
            notification_api,
        }
    }
}

#[async_trait]
impl<Q, S, N> ApplicationApi for Subito<Q, S, N>
where
    Q: QueryApi + Sync + Send,
    S: ScraperApi + Sync + Send,
    N: NotificationApi + Sync + Send,
{
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.query_api.lock().await.add_search(Arc::new(Search::new(name, query))).await
    }

    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.query_api.lock().await.delete_search(name).await
    }

    async fn list(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>> {
        self.query_api.lock().await.fetch_all_searches()
    }

    async fn scrape(&self) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>> {
        let mut results: Vec<Arc<ItemResult>> = vec![];
        let searches = self.query_api.lock().await.fetch_all_searches()?;

        for search in searches {
            let mut scrape_results = self.scraper_api.run_query(Arc::clone(&search)).await?;
            results.append(&mut scrape_results)
        }

        let items = self.query_api.lock().await.fetch_all_items()?;

        for result in &results {
            if !items.contains(&result.get_uri()) {
                self.notification_api.notify(format!("{result}")).await?;
            }
        }

        Ok(results)
    }
}
