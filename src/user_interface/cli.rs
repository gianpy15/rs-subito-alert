use std::{error::Error, process::exit, sync::Arc};

use crate::{
    application::application_api::ApplicationApi,
    telegram_bot::{telegram_bot_agent::TelegramBotAgent, telegram_bot_api::TelegramBotApi},
    types::Application,
};
use async_trait::async_trait;
use inquire::{parse_type, parser::CustomTypeParser, validator::Validation, Confirm, Select, Text};

use super::{options::Options, user_interface_api::UserInterfaceApi};

pub struct Cli {
    application: Application,
    bot_agent: TelegramBotAgent,
}

impl Cli {
    pub fn new(application: Application, bot_agent: TelegramBotAgent) -> Self {
        Self {
            application,
            bot_agent,
        }
    }
}

#[async_trait]
impl UserInterfaceApi for Cli {
    async fn start_cli(&self) {
        loop {
            let options = vec![
                Options::Start,
                Options::ApiKey,
                Options::ScrapeInterval,
                Options::Reset,
                Options::Quit,
            ];
            let option = Select::new("Select an action:", options).prompt();
            match option {
                Ok(Options::ApiKey) => {
                    let api_key = Text::new("Insert Telegram api_key>").prompt().unwrap();
                    let _ = self.add_api_key(&api_key).await;
                    println!("Please restart application.");
                    self.quit();
                }
                Ok(Options::Start) => {
                    let _ = self.start_application().await;
                }
                Ok(Options::ScrapeInterval) => {
                    let validator = |input: &str| {
                        let parser: CustomTypeParser<i32> = parse_type!(i32);
                        let val: Result<Validation, Box<dyn Error + Send + Sync + 'static>> =
                            match parser(input) {
                                Ok(interval) => {
                                    if interval > 0 {
                                        Ok(Validation::Valid)
                                    } else {
                                        Ok(Validation::Invalid(
                                            "Please insert a number > 0.".into(),
                                        ))
                                    }
                                }
                                Err(_) => {
                                    Ok(Validation::Invalid("Please insert a number > 0.".into()))
                                }
                            };

                        val
                    };
                    let old_interval = self
                        .application
                        .lock()
                        .await
                        .get_scraping_interval()
                        .await
                        .unwrap_or_default();
                    let interval = Text::new("Insert scrape interval (s)")
                        .with_default(&old_interval.to_string())
                        .with_validator(validator)
                        .prompt()
                        .unwrap();
                    let interval_value: i32 = interval.parse().unwrap();
                    let _ = self.set_scrape_interval(interval_value).await;
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

    async fn add_api_key(&self, api_key: &str) -> Result<(), Box<dyn Error>> {
        self.bot_agent.add_api_key(api_key).await
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
        let _ = tokio::join!(scraper, self.bot_agent.start(Arc::clone(&self.application)));
        Ok(())
    }

    async fn reset_application(&self) -> Result<(), Box<dyn Error>> {
        println!("Resetting...");
        self.application.lock().await.reset().await?;
        println!("Done!");
        Ok(())
    }

    async fn set_scrape_interval(&self, interval: i32) -> Result<(), Box<dyn Error>> {
        self.application
            .lock()
            .await
            .set_scraping_interval(interval)
            .await?;
        println!("Done!");
        Ok(())
    }

    fn quit(&self) -> ! {
        println!("Quitting...");
        exit(0)
    }
}
