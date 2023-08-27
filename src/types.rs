use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::subito::Subito,
    notification::telegram_notifier::TelegramNotifier,
    query_db::query_engine::QueryEngine,
    scraper::{downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent},
    serializer::serializer_agent::SerializerAgent,
};

pub type Application = Arc<
    Mutex<
        Subito<
            QueryEngine<SerializerAgent>,
            ScraperAgent<DownloadAgent>,
            TelegramNotifier<SerializerAgent>,
        >,
    >,
>;
