use std::error::Error;

use crate::query_db::db::DataBase;

pub trait SerializerApi {
    fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn Error>>;
}
