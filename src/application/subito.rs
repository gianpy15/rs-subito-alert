use std::{borrow::BorrowMut, error::Error};

use crate::{
    notification::notification_api::NotificationApi,
    query_db::{query::QueryApi, search::Search},
    scraper::{item_result::ItemResult, scraper_api::ScraperApi},
};

use super::application_api::ApplicationApi;

struct Subito<'a, Q, S, N> {
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
        self.query_api.add_search(Search::new(name, query))
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.query_api.delete_search(name)
    }

    fn list(&mut self) -> Result<Vec<Search>, Box<dyn Error>> {
        self.query_api.fetch_all_searches()
    }

    fn scrape(&mut self) -> Result<Vec<ItemResult>, Box<dyn Error>> {
        let mut results: Vec<ItemResult> = vec![];
        let searches = self.query_api.fetch_all_searches()?;

        for search in searches {
            let mut scrape_results = self.scraper_api.run_query(search)?;
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

#[cfg(test)]
mod tests {

    use crate::testing::{
        notifier::NotifierSpy,
        query::{QueryDbFake, QueryDbSpy},
        scraper::{ScraperFake, ScraperSpy},
    };

    use super::*;

    #[test]
    fn test_add_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut scraper = ScraperFake {};
        let mut notifier_spy = NotifierSpy::default();
        let mut subito = Subito::new(&mut query_spy, &mut scraper, &mut notifier_spy);

        let _ = subito.add_search(String::from("Test"), String::from("test"));

        assert_eq!(
            query_spy.invocations,
            vec![Search::new(String::from("Test"), String::from("test"))]
        )
    }

    #[test]
    fn test_delete_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut scraper = ScraperFake {};
        let mut notifier_spy = NotifierSpy::default();
        let mut subito = Subito::new(&mut query_spy, &mut scraper, &mut notifier_spy);

        let _ = subito.delete_search(String::from("Test"));

        assert_eq!(query_spy.deletions, vec![String::from("Test")])
    }

    #[test]
    fn test_list_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut scraper = ScraperFake {};
        let mut notifier_spy = NotifierSpy::default();
        let mut subito = Subito::new(&mut query_spy, &mut scraper, &mut notifier_spy);

        let _ = subito.list();

        assert_eq!(query_spy.lists, vec![()])
    }

    #[test]
    fn test_scrape() -> Result<(), Box<dyn Error>> {
        let mut scraper_spy = ScraperSpy::new();
        let mut query_fake = QueryDbFake::new();
        let mut notifier_spy = NotifierSpy::default();
        let mut subito = Subito::new(&mut query_fake, &mut scraper_spy, &mut notifier_spy);

        let _ = subito.scrape();

        assert_eq!(scraper_spy.invocations, 3);
        Ok(())
    }

    #[test]
    fn test_scrape_results() -> Result<(), Box<dyn Error>> {
        let mut scraper_spy = ScraperSpy::new();
        let mut query_fake = QueryDbFake::new();
        let mut notifier_spy = NotifierSpy::default();
        let mut subito = Subito::new(&mut query_fake, &mut scraper_spy, &mut notifier_spy);

        let results = subito.scrape()?;

        assert_eq!(scraper_spy.invocations, (results.len() as i32));
        Ok(())
    }

    #[test]
    fn test_notification_on_new_items() -> Result<(), Box<dyn Error>> {
        let mut scraper_spy = ScraperSpy::new();
        let mut query_fake = QueryDbFake::new();
        let mut notifier_spy = NotifierSpy::default();
        let mut subito = Subito::new(&mut query_fake, &mut scraper_spy, &mut notifier_spy);

        let results = subito.scrape()?;

        assert_eq!(notifier_spy.invocations, (results.len() as i32) - 2);
        Ok(())
    }
}
