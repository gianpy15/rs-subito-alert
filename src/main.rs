use std::{env, sync::Arc};

use rs_subito_alert::{
    application::subito::Subito,
    notification::telegram_notifier::TelegramNotifier,
    query_db::query_engine::QueryEngine,
    scraper::{downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent},
    serializer::{serializer_agent::SerializerAgent, serializer_api::SerializerApi},
    telegram_bot::env::TelegramEnvironment,
    user_interface::{cli::Cli, user_interface_api::UserInterfaceApi},
};
use teloxide::prelude::*;
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

    let (env_serializer, db_serializer) = tokio::join!(
        SerializerAgent::new(String::from("telegram.json"), None),
        SerializerAgent::new(String::from("database.json"), None)
    );
    let env: TelegramEnvironment = env_serializer
        .deserialize()
        .await
        .ok()
        .unwrap_or(TelegramEnvironment::default());
    let bot = Arc::new(Bot::new(env.get_token()));
    let application = Arc::new(Mutex::new(build_app(Arc::clone(&bot)).await));
    let cli = Cli::new(
        Arc::clone(&application),
        env_serializer,
        db_serializer,
        Arc::clone(&bot),
    );

    let args: Vec<String> = env::args().collect();
    let skip_cli: bool = match args.get(1) {
        Some(arg) => [String::from("--skip-dialogue"), String::from("-s")].contains(arg),
        _ => false,
    };

    if skip_cli {
        let _ = cli.start_application().await;
    } else {
        cli.start_cli().await;
    }
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
