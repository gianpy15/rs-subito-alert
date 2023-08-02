use std::io::Error;

use crate::query_db::{query::QueryApi, search::Search};

trait Application {
    fn add_search(&mut self, name: String, query: String) -> Result<(), Error>;
    fn delete_search(&mut self, name: String) -> Result<(), Error>;
    fn list(&mut self) -> Result<String, Error>;
    fn scrape(&mut self) -> Result<(), Error>;
}

struct Subito<'a, Q> {
    query_api: &'a mut Q
}

impl<'a, Q> Subito<'a, Q>
where
    Q: QueryApi,
{
    pub fn new(query_api: &'a mut Q) -> Subito<'a, Q> {
        Subito {query_api}
    }
}

impl<'a, Q> Application for Subito<'a, Q>
where
    Q: QueryApi,
{
    fn add_search(&mut self, name: String, query: String) -> Result<(), Error> {
        self.query_api.add_search(Search::new(name, query))?;
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Error> {
        todo!()
    }

    fn list(&mut self) -> Result<String, Error> {
        todo!()
    }

    fn scrape(&mut self) -> Result<(), Error> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::query_db::search::Search;

    use super::*;

    struct QueryDbSpy {
        invocations: Vec<Search>
    }

    impl QueryDbSpy {
        pub fn new() -> QueryDbSpy {
            QueryDbSpy { invocations: vec![] }
        }

        pub fn get_invocations(&self) -> Vec<Search> {
            return self.invocations.clone();
        }
    }

    impl QueryApi for QueryDbSpy {
        fn add_search(&mut self, search: Search) -> Result<(), Error> {
            self.invocations.push(search);
            Ok(())
        }
    }

    #[test]
    fn test_add_search() {
        let mut query_spy = QueryDbSpy::new();
        let mut subito = Subito::new(&mut query_spy);
        
        subito.add_search(String::from("Test"), String::from("test"));
        
        assert_eq!(query_spy.get_invocations(), vec![Search::new(String::from("Test"), String::from("test"))])

    }

    fn test_delete_search() {

    }

    fn test_list_search() {

    }

    fn test_scrape() {

    }


}