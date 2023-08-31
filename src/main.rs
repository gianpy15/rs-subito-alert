use std::{env, sync::Arc};

use rs_subito_alert::{
    application::subito::Subito,
    notification::telegram_notifier::TelegramNotifier,
    query_db::query_engine::QueryEngine,
    scraper::{downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent},
    serializer::serializer_agent::SerializerAgent,
    telegram_bot::{telegram_bot_agent::TelegramBotAgent, telegram_bot_api::TelegramBotApi},
    user_interface::{cli::Cli, user_interface_api::UserInterfaceApi},
};
use teloxide::{adaptors::DefaultParseMode, prelude::*};
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

    let env_serializer = SerializerAgent::new("telegram.json", None).await;
    let bot_agent = TelegramBotAgent::new(env_serializer.clone()).await;
    let bot = bot_agent.get_bot();
    let application = Arc::new(Mutex::new(build_app(Arc::clone(&bot)).await));
    let cli = Cli::new(Arc::clone(&application), bot_agent);

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

async fn build_app(bot: Arc<DefaultParseMode<Bot>>) -> Application {
    let env_serializer = Arc::new(SerializerAgent::new("telegram.json", None).await);
    let db_serializer = Arc::new(SerializerAgent::new("database.json", None).await);
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
