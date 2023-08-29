use std::error::Error;

use async_trait::async_trait;
use rs_subito_alert::{
    serializer::serializer_api::SerializerApi, telegram_bot::env::TelegramEnvironment,
};

pub struct SerializerDouble {}

impl SerializerDouble {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SerializerApi<TelegramEnvironment> for SerializerDouble {
    async fn serialize(&self, _obj: &TelegramEnvironment) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn deserialize(&self) -> Result<TelegramEnvironment, Box<dyn Error>> {
        Ok(TelegramEnvironment::new(String::from("api_key")))
    }

    async fn clear(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
