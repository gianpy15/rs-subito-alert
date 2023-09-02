use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{env::temp_dir, error::Error, path::PathBuf};
use tokio::fs;

use super::serializer_api::SerializerApi;

#[derive(Clone)]
pub struct SerializerAgent {
    base_path: PathBuf,
    fname: String,
}

impl SerializerAgent {
    pub async fn new(fname: &str, sub_path: Option<&str>) -> Self {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("subito-alert");
        if let Some(p) = sub_path {
            config_dir.push(p)
        }
        fs::create_dir_all(&config_dir).await.ok().unwrap();
        Self {
            base_path: config_dir,
            fname: String::from(fname),
        }
    }

    pub async fn build_with_test_dir(fname: String) -> Self {
        let mut config_dir = temp_dir();
        config_dir.push("subito-alert");
        fs::create_dir_all(&config_dir).await.ok().unwrap();
        Self {
            base_path: config_dir,
            fname,
        }
    }

    pub fn get_full_path(&self) -> PathBuf {
        let mut file_path = self.base_path.clone();
        file_path.push("subito-alert");
        file_path.set_file_name(&self.fname);
        file_path
    }
}

#[async_trait]
impl<T> SerializerApi<T> for SerializerAgent
where
    T: Serialize + DeserializeOwned + Sync,
{
    async fn serialize(&self, obj: &T) -> Result<(), Box<dyn Error>> {
        let file_path = self.get_full_path();

        let serialized = serde_json::to_string(obj)?;

        fs::write(file_path, serialized).await?;
        Ok(())
    }

    async fn deserialize(&self) -> Result<T, Box<dyn Error>> {
        let file_path = self.get_full_path();

        let obj_string = fs::read_to_string(file_path).await?;
        let obj: T = serde_json::from_str(&obj_string)?;
        Ok(obj)
    }

    async fn clear(&self) -> Result<(), Box<dyn Error>> {
        fs::remove_file(self.get_full_path()).await?;
        let _ = fs::remove_dir(self.get_full_path().parent().unwrap()).await;
        Ok(())
    }
}
