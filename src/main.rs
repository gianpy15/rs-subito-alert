use std::sync::Mutex;
use std::{env, error::Error};
use rs_subito_alert::application;
use rs_subito_alert::application::application_api::ApplicationApi;
use rs_subito_alert::application::subito::Subito;
use rs_subito_alert::notification::telegram_notifier::TelegramNotifier;
use rs_subito_alert::query_db::query_engine::QueryEngine;
use rs_subito_alert::serializer::serializer_agent::SerializerAgent;
use rs_subito_alert::serializer::serializer_api::SerializerApi;
use rs_subito_alert::telegram_bot::env::TelegramEnvironment;
use teloxide::prelude::*;
use rs_subito_alert::telegram_bot::commands::Command;

use rs_subito_alert::{
    query_db::search::Search,
    scraper::{
        downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent,
        scraper_api::ScraperApi,
    },
};
use teloxide::utils::command::BotCommands;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let mut env_serializer = SerializerAgent::new(String::from("telegram.json"));
    let env: TelegramEnvironment = env_serializer.deserialize().ok().unwrap();
    let bot = Bot::new(env.get_token());

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    let message_str = {
        let env_serializer = SerializerAgent::new(String::from("telegram.json"));

        let mut serializer = SerializerAgent::default();
        let mut query_api = QueryEngine::new(&mut serializer);
        let download_api = DownloadAgent::default();
        let mut scraper_api = ScraperAgent::new(&download_api);
        let mut notification_api = TelegramNotifier::new(env_serializer);
        let mut application = Subito::new(&mut query_api, &mut scraper_api, &mut notification_api);
        
        match command {
        Command::Help => {
            Command::descriptions().to_string()
        }
        Command::List => {
            let searches = application.list().ok().unwrap();
            format!("{:?}", searches)
        }
        Command::Add { name, query } => {
            application.add_search(name, query);
            String::from("Add")
        }
    }};
    bot.send_message(message.chat.id, message_str).await?;

    Ok(())
}


async fn test_telegram_bot() {
    env::set_var("TELOXIDE_TOKEN", "");

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::new("");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}

fn test_scraper() -> Result<(), Box<dyn Error>> {
    let download: DownloadAgent = Default::default();
    let mut scraper = ScraperAgent::new(&download);

    let results = scraper.run_query(
        Search {
            name: "Test".to_string().into(),
            query: "Zelda Tears of the kingdom".to_string().into(),
        }
        .into(),
    )?;

    for result in results {
        println!("{}", result)
    }

    Ok(())
}
