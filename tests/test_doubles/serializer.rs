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
    async fn serialize(
        &mut self,
        obj: &TelegramEnvironment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn deserialize(&mut self) -> Result<TelegramEnvironment, Box<dyn std::error::Error>> {
        Ok(TelegramEnvironment::new(String::from("api_key")))
    }
}
