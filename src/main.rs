use std::{env, error::Error, thread, time};
use rs_subito_alert::application::subito::Subito;
use rs_subito_alert::query_db::query_engine::QueryEngine;
use rs_subito_alert::scraper::downloader::download_api;
use rs_subito_alert::serializer::serializer_agent::SerializerAgent;
use rs_subito_alert::telegram_bot::telegram_bot_agent::TelegramBotAgent;
use teloxide::prelude::*;
use rs_subito_alert::telegram_bot::commands::Command;

use rs_subito_alert::{
    query_db::search::Search,
    scraper::{
        downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent,
        scraper_api::ScraperApi,
    },
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    // let bot = Bot::from_env();

    // let mut serializer = SerializerAgent::default();
    // let query_api = QueryEngine::new(database, &mut serializer);
    // let download_api = DownloadAgent::default();
    // let scraper_api = ScraperAgent::new(&download_api);
    // let application = Subito::new(query_api, scraper_api, notification_api)

    // let telegram_agent = TelegramBotAgent::new(application, serializer)

    // Command::repl(bot, answer).await;
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
