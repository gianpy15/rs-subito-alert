use rs_subito_alert::application::application_api::ApplicationApi;
use rs_subito_alert::application::subito::Subito;
use rs_subito_alert::notification::telegram_notifier::TelegramNotifier;
use rs_subito_alert::query_db::query_engine::QueryEngine;

use rs_subito_alert::scraper::{
    downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent,
};
use rs_subito_alert::serializer::serializer_agent::SerializerAgent;
use rs_subito_alert::serializer::serializer_api::SerializerApi;
use rs_subito_alert::telegram_bot::commands::Command;
use rs_subito_alert::telegram_bot::env::TelegramEnvironment;
use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use tokio::sync::Mutex;

type Application = Subito<
    QueryEngine<SerializerAgent>,
    ScraperAgent<DownloadAgent>,
    TelegramNotifier<SerializerAgent>,
>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let env_serializer = SerializerAgent::new(String::from("telegram.json"), None).await;
    let env: TelegramEnvironment = env_serializer.deserialize().await.ok().unwrap();
    let bot = Arc::new(Bot::new(env.get_token()));
    let application = Arc::new(Mutex::new(build_app(Arc::clone(&bot)).await));

    let scraper_app = Arc::clone(&application);
    let scraper = tokio::spawn(async move {
        log::info!("Starting scraper...");
        loop {
            let _ = scraper_app.lock().await.scrape().await;
            log::info!("Scraped...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            log::info!("Waited...");
        }
    });

    let bot_handler = Command::repl(Arc::clone(&bot), move |a, b, c| {
        let app = Arc::clone(&application);
        async move { answer(a, b, c, app).await }
    });

    let _ = tokio::join!(scraper, bot_handler);
}

async fn build_app(bot: Arc<Bot>) -> Application {
    let env_serializer = Arc::new(SerializerAgent::new(String::from("telegram.json"), None).await);
    let db_serializer = Arc::new(SerializerAgent::new(String::from("database.json"), None).await);
    let query_api = Arc::new(Mutex::new(
        QueryEngine::new(Arc::clone(&db_serializer)).await,
    ));
    let download_api = Arc::new(DownloadAgent::default());
    let scraper_api = Arc::new(ScraperAgent::new(Arc::clone(&download_api)));
    let notification_api = Arc::new(TelegramNotifier::new(
        Arc::clone(&env_serializer),
        Arc::clone(&bot),
    ));
    Subito::new(
        Arc::clone(&query_api),
        Arc::clone(&scraper_api),
        Arc::clone(&notification_api),
    )
}

async fn answer(
    bot: Arc<Bot>,
    message: Message,
    command: Command,
    application: Arc<Mutex<Application>>,
) -> ResponseResult<()> {
    let _message_str = {
        match command {
            Command::Help => {
                bot.send_message(message.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::List => {
                let searches = application.lock().await.list().await.unwrap();
                bot.send_message(message.chat.id, format!("{:?}", searches))
                    .await?
            }
            Command::Add { name, query } => {
                let _ = application.lock().await.add_search(name, query).await;

                bot.send_message(message.chat.id, "Add").await?
            }
        }
    };

    Ok(())
}
