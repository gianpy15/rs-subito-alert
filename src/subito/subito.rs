use std::io::Error;

use crate::query_db::{query::QueryApi, search::Search};

use super::application::Application;

struct Subito<'a, Q> {
    query_api: &'a mut Q,
}

impl<'a, Q> Subito<'a, Q>
where
    Q: QueryApi,
{
    pub fn new(query_api: &'a mut Q) -> Subito<'a, Q> {
        Subito { query_api }
    }
}

impl<'a, Q> Application for Subito<'a, Q>
where
    Q: QueryApi,
{
    fn add_search(&mut self, name: String, query: String) -> Result<(), Error> {
        self.query_api.add_search(Search::new(name, query))
    }

    fn delete_search(&mut self, name: String) -> Result<(), Error> {
        self.query_api.delete_search(name)
    }

    fn list(&mut self) -> Result<Vec<Search>, Error> {
        self.query_api.fetch_all()
    }

    fn scrape(&mut self) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::subito::testing::QueryDbSpy;

    use super::*;

    #[test]
    fn test_add_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut subito = Subito::new(&mut query_spy);

        subito.add_search(String::from("Test"), String::from("test"));

        assert_eq!(
            query_spy.invocations,
            vec![Search::new(String::from("Test"), String::from("test"))]
        )
    }

    #[test]
    fn test_delete_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut subito = Subito::new(&mut query_spy);

        subito.delete_search(String::from("Test"));

        assert_eq!(query_spy.deletions, vec![String::from("Test")])
    }

    #[test]
    fn test_list_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut subito = Subito::new(&mut query_spy);

        subito.list();

        assert_eq!(query_spy.lists, vec![()])
    }

    #[test]
    fn test_scrape() {}
}
