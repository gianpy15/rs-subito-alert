use std::{error::Error, rc::Rc};

use rs_subito_alert::{
    query_db::{db::DataBase, query_api::QueryApi, query_engine::QueryEngine, search::Search},
    scraper::item_result::ItemResult,
    serializer::serializer_api::SerializerApi,
};

struct SerializerSpy {
    invocations: Vec<Option<DataBase>>,
}

impl SerializerSpy {
    fn new() -> Self {
        Self {
            invocations: vec![],
        }
    }
}

impl SerializerApi<DataBase> for SerializerSpy {
    fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn Error>> {
        self.invocations.push(Some(database.clone()));
        Ok(())
    }

    fn deserialize(&mut self) -> Result<DataBase, Box<dyn Error>> {
        self.invocations.push(None);
        Ok(Default::default())
    }
}

#[test]
fn test_add_to_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database, &mut serializer_spy);

    query_engine.add_search(Search::new("Test".to_string(), "test".to_string()).into())?;

    assert_eq!(
        query_engine.database,
        DataBase::new(
            vec![Rc::new(Search::new("Test".to_string(), "test".to_string()))],
            vec![]
        )
    );
    Ok(())
}

#[test]
fn test_serialize_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine.add_search(Search::new("Test".to_string(), "test".to_string()).into())?;

    assert_eq!(
        serializer_spy.invocations,
        vec![Some(DataBase::new(
            vec![Rc::new(Search::new("Test".to_string(), "test".to_string()))],
            vec![]
        ))]
    );
    Ok(())
}

#[test]
fn test_delete_search() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

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
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine.add_search(Rc::new(Search::new("Test".to_string(), "test".to_string())))?;
    query_engine.add_search(Rc::new(Search::new(
        "Test2".to_string(),
        "test2".to_string(),
    )))?;
    let mut result = query_engine.fetch_all_searches()?;

    result.sort();

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
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine.add_items(vec![
        ItemResult::default("a", "a"),
        ItemResult::default("b", "b"),
    ])?;
    let mut result = query_engine.fetch_all_items()?;
    result.sort();

    assert_eq!(
        result,
        vec![Rc::new(String::from("a")), Rc::new(String::from("b"))]
    );
    Ok(())
}
