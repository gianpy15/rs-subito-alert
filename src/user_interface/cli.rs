use std::{error::Error, process::exit, sync::Arc};

use crate::{
    application::application_api::ApplicationApi,
    serializer::serializer_api::SerializerApi,
    telegram_bot::{env::TelegramEnvironment, handlers::bot_handlers, state::State},
    types::Application,
};
use async_trait::async_trait;
use inquire::{Select, Text};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio::sync::Mutex;

use super::{options::Options, user_interface_api::UserInterfaceApi};

pub struct Cli<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    application: Application,
    serializer: S,
    bot: Arc<Bot>,
}

impl<S> Cli<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    pub fn new(application: Application, serializer: S, bot: Arc<Bot>) -> Self {
        Self {
            application,
            serializer,
            bot,
        }
    }
}

#[async_trait]
impl<S> UserInterfaceApi for Cli<S>
where
    S: SerializerApi<TelegramEnvironment> + Send + Sync,
{
    async fn start_cli(&self) {
        loop {
            let options = vec![Options::Start, Options::ApiKey, Options::Quit];
            let option = Select::new("Select an action:", options).prompt();
            match option {
                Ok(Options::ApiKey) => {
                    let api_key = Text::new("Insert Telegram api_key>").prompt().unwrap();
                    let _ = self.add_api_key(api_key).await;
                }
                Ok(Options::Start) => {
                    let _ = self.start_application().await;
                }
                _ => {
                    self.quit();
                }
            };
        }
    }

    async fn add_api_key(&self, api_key: String) -> Result<(), Box<dyn Error>> {
        let mut env = self.serializer.deserialize().await?;
        env.set_token(api_key);
        self.serializer.serialize(&env).await?;
        Ok(())
    }

    async fn start_application(&self) -> Result<(), Box<dyn Error>> {
        println!("Application started");
        Dispatcher::builder(
            Arc::clone(&self.bot),
            bot_handlers::schema(Arc::clone(&self.application)).await,
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
        Ok(())
    }

    fn quit(&self) {
        println!("Quitting...");
        exit(0)
    }
}
