use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{env::temp_dir, error::Error, path::PathBuf};
use tokio::fs;

use super::serializer_api::SerializerApi;

pub struct SerializerAgent {
    base_path: PathBuf,
    fname: String,
}

impl SerializerAgent {
    pub async fn new(fname: String, sub_path: Option<String>) -> Self {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("subito-alert");
        sub_path.and_then(|p| Some(config_dir.push(p)));
        fs::create_dir_all(&config_dir).await.ok().unwrap();
        Self {
            base_path: config_dir,
            fname,
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
    async fn serialize(&mut self, obj: &T) -> Result<(), Box<dyn Error>> {
        let file_path = self.get_full_path();

        let serialized = serde_json::to_string(obj)?;

        fs::write(file_path, serialized).await?;
        Ok(())
    }

    async fn deserialize(&mut self) -> Result<T, Box<dyn std::error::Error>> {
        let file_path = self.get_full_path();

        let obj_string = fs::read_to_string(file_path).await?;
        let obj: T = serde_json::from_str(&obj_string)?;
        Ok(obj)
    }
}
