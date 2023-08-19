use std::{env, error::Error, thread, time};
use teloxide::prelude::*;

use rs_subito_alert::{
    query_db::search::Search,
    scraper::{
        downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent,
        scraper_api::ScraperApi,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_telegram_bot();

    loop {
        thread::sleep(time::Duration::from_millis(1000));
        println!("here");
    }
    Ok(())
}

async fn test_telegram_bot() {
    env::set_var(
        "TELOXIDE_TOKEN",
        "",
    );

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
