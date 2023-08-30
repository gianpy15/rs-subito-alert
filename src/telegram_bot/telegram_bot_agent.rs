use crate::application::application_api::ApplicationApi;

use async_trait::async_trait;
use std::error::Error;
use teloxide::prelude::*;

use super::commands::Command;

use super::telegram_bot_api::TelegramBotApi;

pub struct TelegramBotAgent<'a, S> {
    subito: &'a mut S,
}

#[async_trait]
impl<'a, S> TelegramBotApi for TelegramBotAgent<'a, S>
where
    S: ApplicationApi + Send + Sync,
{
    async fn add_search(&mut self, name: &str, query: &str) -> Result<(), Box<dyn Error>> {
        self.subito.add_search(name, query).await?;
        Ok(())
    }
    async fn list_searches(&mut self) -> Result<(), Box<dyn Error>> {
        let _searches = self.subito.list().await?;
        Ok(())
    }
}

impl<'a, S> TelegramBotAgent<'a, S>
where
    S: ApplicationApi + Send + Sync,
{
    pub fn new(application: &'a mut S) -> Self {
        Self {
            subito: application,
        }
    }

    pub async fn start(&mut self, _bot: Bot, _msg: Message, _cmd: Command) -> ResponseResult<()> {
        todo!()
    }
}
