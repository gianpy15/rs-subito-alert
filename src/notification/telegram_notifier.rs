use async_trait::async_trait;
use teloxide::{Bot, requests::Requester};

use crate::{serializer::serializer_api::SerializerApi, telegram_bot::env::TelegramEnvironment, scraper::item_result::ItemResult};

use super::notification_api::NotificationApi;

pub struct TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    serializer: S,
    telegram_bot: Bot
}

impl<S> TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    pub fn new(mut serializer: S) -> Self {
        let bot = Bot::new(serializer.deserialize().ok().unwrap().get_token());
        Self { serializer, telegram_bot: bot }
    }
}

#[async_trait]
impl<S> NotificationApi for TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment> + Send,
{
    async fn notify(
        &mut self,
        item: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let chat_ids = self.serializer.deserialize().ok().unwrap().get_chat_ids();
        for x in chat_ids {
            let y = self.telegram_bot.send_message(x, &item).await;
        }
        Ok(())
    }
}
