use crate::{serializer::serializer_api::SerializerApi, telegram_bot::env::TelegramEnvironment};

use super::notification_api::NotificationApi;

pub struct TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>
{
    serializer: S
}

impl<S> TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>
{
    pub fn new(serializer: S) -> Self {
        Self { serializer }
    }
}

impl<S> NotificationApi for TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>
{
    fn notify(&mut self, item: &crate::scraper::item_result::ItemResult) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", item);
        Ok(())
    }
}