use std::{error::Error, rc::Rc};

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

impl<'a, Q, S, N> ApplicationApi for Subito<'a, Q, S, N>
where
    Q: QueryApi,
    S: ScraperApi,
    N: NotificationApi,
{
    fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.query_api.add_search(Rc::new(Search::new(name, query)))
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.query_api.delete_search(name)
    }

    fn list(&mut self) -> Result<Vec<Rc<Search>>, Box<dyn Error>> {
        self.query_api.fetch_all_searches()
    }

    fn scrape(&mut self) -> Result<Vec<Rc<ItemResult>>, Box<dyn Error>> {
        let mut results: Vec<Rc<ItemResult>> = vec![];
        let searches = self.query_api.fetch_all_searches()?;

        for search in searches {
            let mut scrape_results = self.scraper_api.run_query(Rc::clone(&search))?;
            results.append(&mut scrape_results)
        }

        let items = self.query_api.fetch_all_items()?;

        results
            .iter()
            .filter(|result| !items.contains(&result.get_uri()))
            .for_each(|result| {
                self.notification_api.notify(result);
            });

        Ok(results)
    }
}
