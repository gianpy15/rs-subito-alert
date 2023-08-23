use async_trait::async_trait;
use teloxide::{requests::Requester, Bot};

use crate::{serializer::serializer_api::SerializerApi, telegram_bot::env::TelegramEnvironment};

use super::notification_api::NotificationApi;

pub struct TelegramNotifier<'a, S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    serializer: S,
    telegram_bot: &'a Bot,
}

impl<'a, S> TelegramNotifier<'a, S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    pub fn new(mut serializer: S, bot: &'a Bot) -> Self {
        Self {
            serializer,
            telegram_bot: bot,
        }
    }
}

#[async_trait]
impl<'a, S> NotificationApi for TelegramNotifier<'a, S>
where
    S: SerializerApi<TelegramEnvironment> + Send + Sync,
{
    async fn notify(&mut self, item: String) -> Result<(), Box<dyn std::error::Error>> {
        let chat_ids = self
            .serializer
            .deserialize()
            .await
            .ok()
            .unwrap()
            .get_chat_ids();
        for x in chat_ids {
            let _ = self.telegram_bot.send_message(x, &item).await?;
        }
        Ok(())
    }
}
