use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::{
    notification::notification_api::NotificationApi,
    query_db::{query_api::QueryApi, search::Search},
    scraper::{item_result::ItemResult, scraper_api::ScraperApi},
};

use super::application_api::ApplicationApi;

pub struct Subito<'a, Q, S, N> {
    query_api: &'a mut Q,
    scraper_api: &'a mut S,
    notification_api: &'a mut N,
}

impl<'a, Q, S, N> Subito<'a, Q, S, N> {
    pub fn new(
        query_api: &'a mut Q,
        scraper_api: &'a mut S,
        notification_api: &'a mut N,
    ) -> Subito<'a, Q, S, N> {
        Subito {
            query_api,
            scraper_api,
            notification_api,
        }
    }
}

#[async_trait]
impl<'a, Q, S, N> ApplicationApi for Subito<'a, Q, S, N>
where
    Q: QueryApi + Sync + Send,
    S: ScraperApi + Sync + Send,
    N: NotificationApi + Sync + Send,
{
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.query_api
            .add_search(Arc::new(Search::new(name, query)))
            .await
    }

    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.query_api.delete_search(name).await
    }

    fn list(&mut self) -> Result<Vec<Arc<Search>>, Box<dyn Error>> {
        self.query_api.fetch_all_searches()
    }

    async fn scrape(&mut self) -> Result<Vec<Arc<ItemResult>>, Box<dyn Error>> {
        let mut results: Vec<Arc<ItemResult>> = vec![];
        let searches = self.query_api.fetch_all_searches()?;

        for search in searches {
            let mut scrape_results = self.scraper_api.run_query(Arc::clone(&search)).await?;
            results.append(&mut scrape_results)
        }

        let items = self.query_api.fetch_all_items()?;

        for result in &results {
            if !items.contains(&result.get_uri()) {
                self.notification_api.notify(format!("{result}")).await?;
            }
        }

        Ok(results)
    }
}
