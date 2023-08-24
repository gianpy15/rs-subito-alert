use crate::application::application_api::ApplicationApi;

use async_trait::async_trait;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

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
    async fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
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

    pub async fn start(&mut self, bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::List => {
                let _ = self.list_searches().await;
                bot.send_message(msg.chat.id, "List").await?
            }
            Command::Add { name, query } => {
                let _ = self.add_search(name, query).await;
                bot.send_message(msg.chat.id, "Add").await?
            }
        };

        Ok(())
    }
}
