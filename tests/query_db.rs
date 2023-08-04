use std::{error::Error, rc::Rc};

use rs_subito_alert::{
    query_db::{
        db::{DataBase, SerializerApi},
        query::QueryApi,
        query_engine::QueryEngine,
        search::Search,
    },
    scraper::item_result::ItemResult,
};

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

    query_engine.add_search(Search::new("Test".to_string(), "test".to_string()).into())?;

    assert_eq!(
        database,
        DataBase::new(
            vec![Rc::new(Search::new("Test".to_string(), "test".to_string()))],
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

    query_engine.add_search(Search::new("Test".to_string(), "test".to_string()).into())?;

    assert_eq!(
        serializer_spy.invocations,
        vec![DataBase::new(
            vec![Rc::new(Search::new("Test".to_string(), "test".to_string()))],
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

    query_engine.add_search(Rc::new(Search::new("Test".to_string(), "test".to_string())))?;
    query_engine.add_search(Rc::new(Search::new(
        "Test2".to_string(),
        "test2".to_string(),
    )))?;
    query_engine.delete_search("Test".to_string())?;

    assert_eq!(
        query_engine.fetch_all_searches()?,
        vec![Rc::new(Search::new(
            "Test2".to_string(),
            "test2".to_string()
        ))]
    );
    Ok(())
}

#[test]
fn test_fetch_all() -> Result<(), Box<dyn Error>> {
    let mut database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::new(&mut database, &mut serializer_spy);

    query_engine.add_search(Rc::new(Search::new("Test".to_string(), "test".to_string())))?;
    query_engine.add_search(Rc::new(Search::new(
        "Test2".to_string(),
        "test2".to_string(),
    )))?;
    let result = query_engine.fetch_all_searches()?;

    assert_eq!(
        result,
        vec![
            Rc::new(Search::new("Test".to_string(), "test".to_string())),
            Rc::new(Search::new("Test2".to_string(), "test2".to_string()))
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

    assert_eq!(
        result,
        vec![Rc::new(String::from("a")), Rc::new(String::from("b"))]
    );
    Ok(())
}
