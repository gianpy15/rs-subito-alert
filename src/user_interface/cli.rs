use std::{error::Error, process::exit, sync::Arc};

use crate::{
    application::application_api::ApplicationApi,
    serializer::serializer_api::SerializerApi,
    telegram_bot::{env::TelegramEnvironment, handlers::bot_handlers, state::State},
    types::Application,
};
use async_trait::async_trait;
use inquire::{Confirm, Select, Text};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use super::{options::Options, user_interface_api::UserInterfaceApi};

pub struct Cli<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    application: Application,
    env_serializer: S,
    db_serializer: S,
    bot: Arc<Bot>,
}

impl<S> Cli<S>
where
    S: SerializerApi<TelegramEnvironment>,
{
    pub fn new(
        application: Application,
        env_serializer: S,
        db_serializer: S,
        bot: Arc<Bot>,
    ) -> Self {
        Self {
            application,
            env_serializer,
            db_serializer,
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
            let options = vec![
                Options::Start,
                Options::ApiKey,
                Options::Reset,
                Options::Quit,
            ];
            let option = Select::new("Select an action:", options).prompt();
            match option {
                Ok(Options::ApiKey) => {
                    let api_key = Text::new("Insert Telegram api_key>").prompt().unwrap();
                    let _ = self.add_api_key(api_key).await;
                    println!("Please restart application.");
                    self.quit();
                }
                Ok(Options::Start) => {
                    let _ = self.start_application().await;
                }
                Ok(Options::Reset) => {
                    let confirmation = Confirm::new("Are you sure?")
                        .with_default(false)
                        .with_help_message(
                            "This action will delete all the data, configurations and bot chats.",
                        )
                        .prompt();

                    if let Ok(true) = confirmation {
                        let _ = self.reset_application().await;
                        self.quit();
                    }
                }
                _ => {
                    self.quit();
                }
            };
        }
    }

    async fn add_api_key(&self, api_key: String) -> Result<(), Box<dyn Error>> {
        let mut env = self
            .env_serializer
            .deserialize()
            .await
            .unwrap_or(TelegramEnvironment::new("".to_string()));
        env.set_token(api_key);
        self.env_serializer.serialize(&env).await?;
        Ok(())
    }

    async fn start_application(&self) -> Result<(), Box<dyn Error>> {
        let scraper_app = Arc::clone(&self.application);
        let scraper = tokio::spawn(async move {
            log::info!("Starting scraper...");
            loop {
                let _ = scraper_app.lock().await.scrape(None).await;
                log::info!("Scraped...");
                tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
                log::info!("Waited...");
            }
        });
        println!("Application started");
        let mut dispatcher = Dispatcher::builder(
            Arc::clone(&self.bot),
            bot_handlers::schema(Arc::clone(&self.application)).await,
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build();

        let _ = tokio::join!(scraper, dispatcher.dispatch());

        Ok(())
    }

    async fn reset_application(&self) -> Result<(), Box<dyn Error>> {
        println!("Resetting...");
        self.env_serializer.clear().await?;
        self.db_serializer.clear().await?;
        println!("Done!");
        Ok(())
    }

    fn quit(&self) -> ! {
        println!("Quitting...");
        exit(0)
    }
}
