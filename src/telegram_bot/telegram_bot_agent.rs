use crate::application::application_api::ApplicationApi;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use super::commands::Command;

use super::telegram_bot_api::TelegramBotApi;

pub struct TelegramBotAgent<'a, S> {
    subito: &'a mut S,
}

impl<'a, S> TelegramBotApi for TelegramBotAgent<'a, S>
where
    S: ApplicationApi,
{
    fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>> {
        self.subito.add_search(name, query)?;
        Ok(())
    }
    fn list_searches(&mut self) -> Result<(), Box<dyn Error>> {
        let searches = self.subito.list()?;
        Ok(())
    }
}

impl<'a, S> TelegramBotAgent<'a, S>
where
    S: ApplicationApi,
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
                self.list_searches();
                bot.send_message(msg.chat.id, "Ok").await?
            }
            Command::Add { name, query } => {
                self.add_search(name, query);
                bot.send_message(msg.chat.id, "Ok").await?
            }
        };

        Ok(())
    }
}
