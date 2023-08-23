use std::{error::Error, sync::Arc};

use async_trait::async_trait;
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

#[async_trait]
impl SerializerApi<DataBase> for SerializerSpy {
    async fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn Error>> {
        self.invocations.push(Some(database.clone()));
        Ok(())
    }

    async fn deserialize(&mut self) -> Result<DataBase, Box<dyn Error>> {
        self.invocations.push(None);
        Ok(Default::default())
    }
}

#[tokio::test]
async fn test_add_to_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database, &mut serializer_spy);

    query_engine
        .add_search(Search::new("Test".to_string(), "test".to_string()).into())
        .await?;

    assert_eq!(
        query_engine.database,
        DataBase::new(
            vec![Arc::new(Search::new(
                "Test".to_string(),
                "test".to_string()
            ))],
            vec![]
        )
    );
    Ok(())
}

#[tokio::test]
async fn test_serialize_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine
        .add_search(Arc::new(Search::new(
            "Test".to_string(),
            "test".to_string(),
        )))
        .await?;

    assert_eq!(
        serializer_spy.invocations,
        vec![Some(DataBase::new(
            vec![Arc::new(Search::new(
                "Test".to_string(),
                "test".to_string()
            ))],
            vec![]
        ))]
    );
    Ok(())
}

#[tokio::test]
async fn test_delete_search() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine
        .add_search(Arc::new(Search::new(
            "Test".to_string(),
            "test".to_string(),
        )))
        .await?;
    query_engine
        .add_search(Arc::new(Search::new(
            "Test2".to_string(),
            "test2".to_string(),
        )))
        .await?;
    query_engine.delete_search("Test".to_string()).await?;

    assert_eq!(
        query_engine.fetch_all_searches()?,
        vec![Arc::new(Search::new(
            "Test2".to_string(),
            "test2".to_string()
        ))]
    );
    Ok(())
}

#[tokio::test]
async fn test_fetch_all() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine
        .add_search(Arc::new(Search::new(
            "Test".to_string(),
            "test".to_string(),
        )))
        .await;
    query_engine
        .add_search(Arc::new(Search::new(
            "Test2".to_string(),
            "test2".to_string(),
        )))
        .await;

    let mut result = query_engine.fetch_all_searches()?;

    result.sort();

    assert_eq!(
        result,
        vec![
            Arc::new(Search::new("Test".to_string(), "test".to_string())),
            Arc::new(Search::new("Test2".to_string(), "test2".to_string()))
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_fetch_all_items() -> Result<(), Box<dyn Error>> {
    let database: DataBase = Default::default();
    let mut serializer_spy = SerializerSpy::new();
    let mut query_engine = QueryEngine::build(database.clone(), &mut serializer_spy);

    query_engine
        .add_items(vec![
            ItemResult::default("a", "a"),
            ItemResult::default("b", "b"),
        ])
        .await?;
    let mut result = query_engine.fetch_all_items()?;
    result.sort();

    assert_eq!(
        result,
        vec![Arc::new(String::from("a")), Arc::new(String::from("b"))]
    );
    Ok(())
}
