use std::{fs, path::PathBuf};

use html5ever::data;

use crate::query_db::{db::DataBase};

use super::serializer_api::SerializerApi;

pub struct SerializerAgent {
    base_path: PathBuf,
    database_filename: String,
}

impl SerializerAgent {
    pub fn new(base_path: PathBuf, database_filename: String) -> Self {
        fs::create_dir_all(&base_path).ok().unwrap();
        Self {
            base_path,
            database_filename,
        }
    }

    pub fn get_db_path(&self) -> PathBuf {
        let mut file_path = self.base_path.clone();
        file_path.push("subito-alert");
        file_path.set_file_name(&self.database_filename);
        file_path
    }
}

impl Default for SerializerAgent {
    fn default() -> Self {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("subito-alert");

        Self::new(config_dir, String::from("database.json"))
    }
}

impl SerializerApi for SerializerAgent {
    fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.get_db_path();

        let serialized = serde_json::to_string(database)?;
        fs::write(file_path, serialized)?;
        Ok(())
    }

    fn deserialize(&mut self) -> Result<DataBase, Box<dyn std::error::Error>> {
        let file_path = self.get_db_path();

        let db_string = fs::read_to_string(file_path)?;
        Ok(serde_json::from_str(&db_string)?)
    }
}
