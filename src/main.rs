use std::sync::Arc;

use rs_subito_alert::{
    application::subito::Subito,
    notification::telegram_notifier::TelegramNotifier,
    query_db::query_engine::QueryEngine,
    scraper::{downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent},
    serializer::{serializer_agent::SerializerAgent, serializer_api::SerializerApi},
    telegram_bot::{env::TelegramEnvironment, handlers::bot_handlers, state::State},
};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio::sync::Mutex;

type Application = Subito<
    QueryEngine<SerializerAgent>,
    ScraperAgent<DownloadAgent>,
    TelegramNotifier<SerializerAgent>,
>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting purchase bot...");

    let env_serializer = SerializerAgent::new(String::from("telegram.json"), None).await;
    let env: TelegramEnvironment = env_serializer.deserialize().await.ok().unwrap();
    let bot = Arc::new(Bot::new(env.get_token()));
    let application = Arc::new(Mutex::new(build_app(Arc::clone(&bot)).await));

    Dispatcher::builder(
        Arc::clone(&bot),
        bot_handlers::schema(Arc::clone(&application)).await,
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
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
