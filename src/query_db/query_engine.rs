use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    scraper::item_result::ItemResult,
    serializer::{self, serializer_api::SerializerApi},
};

use super::{db::DataBase, query_api::QueryApi, search::Search};

pub struct QueryEngine<S> {
    pub database: DataBase,
    serializer: Arc<Mutex<S>>,
}

impl<S> QueryEngine<S>
where
    S: SerializerApi<DataBase>,
{
    pub async fn new(serializer: Arc<Mutex<S>>) -> QueryEngine<S> {
        let database = match serializer.lock().await.deserialize().await {
            Ok(db) => db,
            Err(_) => DataBase::default(),
        };
        Self {
            database: database,
            serializer,
        }
    }

    pub fn build(database: DataBase, serializer: Arc<Mutex<S>>) -> Self {
        Self {
            database: database,
            serializer,
        }
    }
}

#[async_trait]
impl<S> QueryApi for QueryEngine<S>
where
    S: SerializerApi<DataBase> + Sync + Send,
{
    async fn add_search(&mut self, search: Arc<Search>) -> Result<(), Box<dyn Error>> {
        self.database.add(search);
        self.serializer
            .lock()
            .await
            .serialize(&self.database)
            .await?;
        Ok(())
    }

    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        self.database.delete(name);
        self.serializer
            .lock()
            .await
            .serialize(&self.database)
            .await?;
        Ok(())
    }

    async fn fetch_all_searches(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>> {
        Ok(self.database.get_all_searches())
    }

    async fn fetch_all_items(&self) -> Result<Vec<Arc<String>>, Box<dyn Error>> {
        Ok(self.database.get_all_items())
    }

    async fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        self.database.add_items(items);
        self.serializer
            .lock()
            .await
            .serialize(&self.database)
            .await?;
        Ok(())
    }
}
