use std::error::Error;

use crate::scraper::item_result::ItemResult;

use super::{
    db::{DataBase, SerializerApi},
    query::QueryApi,
    search::Search,
};

pub struct QueryEngine<'a, S> {
    database: &'a mut DataBase,
    serializer: &'a mut S,
}

impl<'a, S> QueryEngine<'a, S> {
    fn new(database: &'a mut DataBase, serializer: &'a mut S) -> Self {
        Self {
            database,
            serializer,
        }
    }
}

impl<'a, S> QueryApi for QueryEngine<'a, S>
where
    S: SerializerApi,
{
    fn add_search(&mut self, search: Search) -> Result<(), Box<dyn Error>> {
        self.database.add(&search);
        self.serializer.serialize(self.database)?;
        Ok(())
    }

    fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.database.delete(name);
        self.serializer.serialize(self.database)?;
        Ok(())
    }

    fn fetch_all_searches(&mut self) -> Result<Vec<Search>, Box<dyn Error>> {
        Ok(self.database.get_all_searches())
    }

    fn fetch_all_items(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self.database.get_all_items())
    }

    fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        self.database.add_items(items);
        self.serializer.serialize(self.database)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SerializerSpy {
        invocations: Vec<DataBase>,
    }

    impl SerializerSpy {
        fn new() -> Self {
            Self {
                invocations: vec![],
            }
        }
    }

    impl SerializerApi for SerializerSpy {
        fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn Error>> {
            self.invocations.push(database.clone());
            Ok(())
        }
    }

    #[test]
    fn test_add_to_db() -> Result<(), Box<dyn Error>> {
        let mut database: DataBase = Default::default();
        let mut serializer_spy = SerializerSpy::new();
        let mut query_engine = QueryEngine::new(&mut database, &mut serializer_spy);

        query_engine.add_search(Search::new("Test".to_string(), "test".to_string()))?;

        assert_eq!(
            database,
            DataBase::new(
                vec![Search::new("Test".to_string(), "test".to_string())],
                vec![]
            )
        );
        Ok(())
    }

    #[test]
    fn test_serialize_db() -> Result<(), Box<dyn Error>> {
        let mut database: DataBase = Default::default();
        let mut serializer_spy = SerializerSpy::new();
        let mut query_engine = QueryEngine::new(&mut database, &mut serializer_spy);

        query_engine.add_search(Search::new("Test".to_string(), "test".to_string()))?;

        assert_eq!(
            serializer_spy.invocations,
            vec![DataBase::new(
                vec![Search::new("Test".to_string(), "test".to_string())],
                vec![]
            )]
        );
        Ok(())
    }

    #[test]
    fn test_delete_search() -> Result<(), Box<dyn Error>> {
        let mut database: DataBase = Default::default();
        let mut serializer_spy = SerializerSpy::new();
        let mut query_engine = QueryEngine::new(&mut database, &mut serializer_spy);

        query_engine.add_search(Search::new("Test".to_string(), "test".to_string()))?;
        query_engine.add_search(Search::new("Test2".to_string(), "test2".to_string()))?;
        query_engine.delete_search("Test".to_string())?;

        assert_eq!(
            query_engine.fetch_all_searches()?,
            vec![Search::new("Test2".to_string(), "test2".to_string())]
        );
        Ok(())
    }

    #[test]
    fn test_fetch_all() -> Result<(), Box<dyn Error>> {
        let mut database: DataBase = Default::default();
        let mut serializer_spy = SerializerSpy::new();
        let mut query_engine = QueryEngine::new(&mut database, &mut serializer_spy);

        query_engine.add_search(Search::new("Test".to_string(), "test".to_string()))?;
        query_engine.add_search(Search::new("Test2".to_string(), "test2".to_string()))?;
        let result = query_engine.fetch_all_searches()?;

        assert_eq!(
            result,
            vec![
                Search::new("Test".to_string(), "test".to_string()),
                Search::new("Test2".to_string(), "test2".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn test_fetch_all_items() -> Result<(), Box<dyn Error>> {
        let mut database: DataBase = Default::default();
        let mut serializer_spy = SerializerSpy::new();
        let mut query_engine = QueryEngine::new(&mut database, &mut serializer_spy);

        query_engine.add_items(vec![
            ItemResult::default("a", "a"),
            ItemResult::default("b", "b"),
        ])?;
        let result = query_engine.fetch_all_items()?;

        assert_eq!(result, vec![String::from("a"), String::from("b")]);
        Ok(())
    }
}
