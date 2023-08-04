use crate::query_db::{serializer::serializer_api::SerializerApi, db::DataBase};

pub struct Serializer {}

impl SerializerApi for Serializer {
    fn serialize(&mut self, database: &DataBase) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}