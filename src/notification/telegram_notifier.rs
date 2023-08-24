use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use teloxide::{requests::Requester, Bot};

use crate::{serializer::serializer_api::SerializerApi, telegram_bot::env::TelegramEnvironment};

use super::notification_api::NotificationApi;

pub struct TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    serializer: Arc<S>,
    telegram_bot: Arc<Bot>,
}

impl<S> TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    pub fn new(serializer: Arc<S>, bot: Arc<Bot>) -> Self {
        Self {
            serializer,
            telegram_bot: bot,
        }
    }
}

#[async_trait]
impl<S> NotificationApi for TelegramNotifier<S>
where
    S: SerializerApi<TelegramEnvironment> + Send + Sync,
{
    async fn notify(&self, item: String) -> Result<(), Box<dyn Error>> {
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
