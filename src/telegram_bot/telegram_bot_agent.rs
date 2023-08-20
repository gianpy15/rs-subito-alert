use crate::application::application_api::ApplicationApi;

use crate::serializer::serializer_api::SerializerApi;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use super::commands::Command;

use super::env::TelegramEnvironment;
use super::telegram_bot_api::TelegramBotApi;

pub struct TelegramBotAgent<'a, S> {
    subito: &'a mut S,
    telegram_bot: Bot,
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
    pub fn new(
        application: &'a mut S,
        serializer: &mut dyn SerializerApi<TelegramEnvironment>,
    ) -> Self {
        let env = serializer.deserialize().ok().unwrap();
        let bot = Bot::new(env.get_token());
        Self {
            subito: application,
            telegram_bot: bot,
        }
    }

    pub async fn start(&mut self, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => {
                self.telegram_bot
                    .send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::List => {
                self.list_searches();
                self.telegram_bot.send_message(msg.chat.id, "List").await?
            }
            Command::Add { name, query } => {
                self.add_search(name, query);
                self.telegram_bot.send_message(msg.chat.id, "Add").await?
            }
        };

        Ok(())
    }
}
