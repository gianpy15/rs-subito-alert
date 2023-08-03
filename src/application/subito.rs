use std::error::Error;

use crate::{
    query_db::{query::QueryApi, search::Search},
    scraper::{item_result::ItemResult, scraper_api::ScraperApi},
};

use super::application_api::ApplicationApi;

struct Subito<'a, Q, S> {
    query_api: &'a mut Q,
    scraper_api: &'a mut S,
}

impl<'a, Q, S> Subito<'a, Q, S>
where
    Q: QueryApi,
    S: ScraperApi,
{
    pub fn new(query_api: &'a mut Q, scraper_api: &'a mut S) -> Subito<'a, Q, S> {
        Subito {
            query_api,
            scraper_api,
        }
    }
}

impl<'a, Q, S> ApplicationApi for Subito<'a, Q, S>
where
    Q: QueryApi,
    S: ScraperApi,
{
    fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.query_api.add_search(Search::new(name, query))
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.query_api.delete_search(name)
    }

    fn list(&mut self) -> Result<Vec<Search>, Box<dyn Error>> {
        self.query_api.fetch_all()
    }

    fn scrape(&mut self, searches: Vec<Search>) -> Result<Vec<ItemResult>, Box<dyn Error>> {
        let mut results: Vec<ItemResult> = vec![];
        for search in searches {
            let mut scrape_results = self.scraper_api.run_query(search)?;
            results.append(&mut scrape_results)
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {

    use crate::testing::{
        query::{QueryDbFake, QueryDbSpy},
        scraper::{ScraperFake, ScraperSpy},
    };

    use super::*;

    #[test]
    fn test_add_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut scraper = ScraperFake {};
        let mut subito = Subito::new(&mut query_spy, &mut scraper);

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
        let mut subito = Subito::new(&mut query_spy, &mut scraper);

        let _ = subito.delete_search(String::from("Test"));

        assert_eq!(query_spy.deletions, vec![String::from("Test")])
    }

    #[test]
    fn test_list_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut scraper = ScraperFake {};
        let mut subito = Subito::new(&mut query_spy, &mut scraper);

        let _ = subito.list();

        assert_eq!(query_spy.lists, vec![()])
    }

    #[test]
    fn test_scrape() {
        let mut scraper_spy = ScraperSpy::new();
        let mut query_fake = QueryDbFake::new();
        let mut subito = Subito::new(&mut query_fake, &mut scraper_spy);

        let searches = vec![
            Search::new("Test".to_string(), "test".to_string()),
            Search::new("Test2".to_string(), "test".to_string()),
            Search::new("Test3".to_string(), "test".to_string()),
        ];

        let _ = subito.scrape(searches);

        assert_eq!(scraper_spy.invocations, 3)
    }

    #[test]
    fn test_scrape_results() -> Result<(), Box<dyn Error>> {
        let mut scraper_spy = ScraperSpy::new();
        let mut query_fake = QueryDbFake::new();
        let mut subito = Subito::new(&mut query_fake, &mut scraper_spy);

        let searches = vec![
            Search::new("Test".to_string(), "test".to_string()),
            Search::new("Test2".to_string(), "test".to_string()),
            Search::new("Test3".to_string(), "test".to_string()),
        ];

        let results = subito.scrape(searches)?;

        assert_eq!(scraper_spy.invocations, (results.len() as i32));
        Ok(())
    }
}
