use std::error::Error;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait SerializerApi<T>
where
    T: Serialize + DeserializeOwned,
{
    async fn serialize(&self, obj: &T) -> Result<(), Box<dyn Error>>;
    async fn deserialize(&self) -> Result<T, Box<dyn Error>>;
}
