use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use rs_subito_alert::{
    query_db::{db::DataBase, query_api::QueryApi, query_engine::QueryEngine, search::Search},
    scraper::item_result::ItemResult,
    serializer::serializer_api::SerializerApi,
};
use tokio::sync::Mutex;

struct SerializerSpy {
    invocations: Mutex<Vec<Option<DataBase>>>,
    database: Mutex<DataBase>,
}

impl SerializerSpy {
    fn new() -> Self {
        Self {
            invocations: Mutex::new(vec![]),
            database: Default::default(),
        }
    }
}

#[async_trait]
impl SerializerApi<DataBase> for SerializerSpy {
    async fn serialize(&self, database: &DataBase) -> Result<(), Box<dyn Error>> {
        let mut db = self.database.lock().await;
        *db = database.clone();
        self.invocations.lock().await.push(Some(database.clone()));
        Ok(())
    }

    async fn deserialize(&self) -> Result<DataBase, Box<dyn Error>> {
        self.invocations.lock().await.push(None);
        let db = self.database.lock().await;
        Ok((*db).clone())
    }

    async fn clear(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[tokio::test]
async fn test_add_to_db() -> Result<(), Box<dyn Error>> {
    let serializer_spy = Arc::new(SerializerSpy::new());
    let mut query_engine = QueryEngine::build(Arc::clone(&serializer_spy));

    query_engine
        .add_search(Search::new("Test", "test").into())
        .await?;

    assert_eq!(
        query_engine.get_database().await,
        DataBase::new(vec![Arc::new(Search::new("Test", "test"))], vec![])
    );
    Ok(())
}

#[tokio::test]
async fn test_serialize_db() -> Result<(), Box<dyn Error>> {
    let serializer_spy = Arc::new(SerializerSpy::new());
    let mut query_engine = QueryEngine::build(Arc::clone(&serializer_spy));

    query_engine
        .add_search(Arc::new(Search::new("Test", "test")))
        .await?;

    assert_eq!(
        *serializer_spy.invocations.lock().await,
        vec![
            None,
            Some(DataBase::new(
                vec![Arc::new(Search::new("Test", "test"))],
                vec![]
            ))
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_delete_search() -> Result<(), Box<dyn Error>> {
    let serializer_spy = Arc::new(SerializerSpy::new());
    let mut query_engine = QueryEngine::build(Arc::clone(&serializer_spy));

    query_engine
        .add_search(Arc::new(Search::new("Test", "test")))
        .await?;
    query_engine
        .add_search(Arc::new(Search::new("Test2", "test2")))
        .await?;
    query_engine.delete_search("Test").await?;

    assert_eq!(
        query_engine.fetch_all_searches().await?,
        vec![Arc::new(Search::new("Test2", "test2"))]
    );
    Ok(())
}

#[tokio::test]
async fn test_fetch_all() -> Result<(), Box<dyn Error>> {
    let serializer_spy = Arc::new(SerializerSpy::new());
    let mut query_engine = QueryEngine::build(Arc::clone(&serializer_spy));

    query_engine
        .add_search(Arc::new(Search::new("Test", "test")))
        .await?;
    query_engine
        .add_search(Arc::new(Search::new("Test2", "test2")))
        .await?;

    let mut result = query_engine.fetch_all_searches().await?;

    result.sort();

    assert_eq!(
        result,
        vec![
            Arc::new(Search::new("Test", "test")),
            Arc::new(Search::new("Test2", "test2"))
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_fetch_all_items() -> Result<(), Box<dyn Error>> {
    let serializer_spy = Arc::new(SerializerSpy::new());
    let mut query_engine = QueryEngine::build(Arc::clone(&serializer_spy));

    query_engine
        .add_items(vec![
            ItemResult::default("a", "a"),
            ItemResult::default("b", "b"),
        ])
        .await?;
    let mut result = query_engine.fetch_all_items().await?;
    result.sort();

    assert_eq!(result, vec![Arc::from("a"), Arc::from("b")]);
    Ok(())
}
