use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use crate::{scraper::item_result::ItemResult, serializer::serializer_api::SerializerApi};

use super::{db::DataBase, query_api::QueryApi, search::Search};

pub struct QueryEngine<S> {
    serializer: Arc<S>,
}

impl<S> QueryEngine<S>
where
    S: SerializerApi<DataBase>,
{
    pub async fn new(serializer: Arc<S>) -> QueryEngine<S> {
        Self { serializer }
    }

    pub fn build(serializer: Arc<S>) -> Self {
        Self { serializer }
    }

    pub async fn get_database(&self) -> DataBase {
        let database = match self.serializer.deserialize().await {
            Ok(db) => db,
            Err(_) => DataBase::default(),
        };
        database
    }
}

#[async_trait]
impl<S> QueryApi for QueryEngine<S>
where
    S: SerializerApi<DataBase> + Sync + Send,
{
    async fn add_search(&mut self, search: Arc<Search>) -> Result<(), Box<dyn Error>> {
        let mut database = self.get_database().await;
        database.add(search);
        self.serializer.serialize(&database).await?;
        Ok(())
    }

    async fn delete_search(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        let mut database = self.get_database().await;
        database.delete(name);
        self.serializer.serialize(&database).await?;
        Ok(())
    }

    async fn fetch_all_searches(&self) -> Result<Vec<Arc<Search>>, Box<dyn Error>> {
        let database = self.get_database().await;
        Ok(database.get_all_searches())
    }

    async fn fetch_all_items(&self) -> Result<Vec<Arc<String>>, Box<dyn Error>> {
        let database = self.get_database().await;
        Ok(database.get_all_items())
    }

    async fn add_items(&mut self, items: Vec<ItemResult>) -> Result<(), Box<dyn Error>> {
        let mut database = self.get_database().await;
        database.add_items(items);
        self.serializer.serialize(&database).await?;
        Ok(())
    }
}
