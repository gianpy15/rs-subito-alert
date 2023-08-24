use rs_subito_alert::application;
use rs_subito_alert::application::application_api::ApplicationApi;
use rs_subito_alert::application::subito::Subito;
use rs_subito_alert::notification::telegram_notifier::TelegramNotifier;
use rs_subito_alert::query_db::query_engine::QueryEngine;
use rs_subito_alert::scraper::downloader::download_api::DownloadApi;
use rs_subito_alert::serializer::serializer_agent::SerializerAgent;
use rs_subito_alert::serializer::serializer_api::SerializerApi;
use rs_subito_alert::telegram_bot::commands::Command;
use rs_subito_alert::telegram_bot::env::TelegramEnvironment;
use rs_subito_alert::{
    query_db::search::Search,
    scraper::{
        downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent,
        scraper_api::ScraperApi,
    },
};
use std::sync::Arc;
use std::{env, error::Error};
use std::{thread, time};
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tokio::sync::mpsc::{self, Receiver, Sender};
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

    Command::repl(Arc::clone(&bot), move |a, b, c| {
        let app = Arc::clone(&application);
        async move { answer(a, b, c, app).await }
    })
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

// let application_handler = thread::spawn(||{
//     let mut i = 0;
//     let env_serializer = SerializerAgent::new(String::from("telegram.json"), None);
//     let mut serializer = SerializerAgent::new(String::from("database.json"), None);
//     let mut query_api = QueryEngine::new(&mut serializer).await;
//     let download_api = DownloadAgent::default();
//     let mut scraper_api = ScraperAgent::new(&download_api);
//     let mut notification_api = TelegramNotifier::new(env_serializer);
//     let mut application = Subito::new(&mut query_api, &mut scraper_api, &mut notification_api);
//     loop {
//         application.scrape();
//         println!("loop {}", i);
//         i += 1;
//         thread::sleep(time::Duration::from_millis(1000));
//     }
// });

//     let env_serializer = SerializerAgent::new(String::from("telegram.json"), None).await;
//     let mut serializer = SerializerAgent::new(String::from("database.json"), None).await;
//     let mut query_api = QueryEngine::new(&mut serializer).await;
//     let download_api = DownloadAgent::default();
//     let mut scraper_api = ScraperAgent::new(&download_api);
//     let mut notification_api = TelegramNotifier::new(env_serializer, &bot);
//     let mut application = Subito::new(&mut query_api, &mut scraper_api, &mut notification_api);

//     let var = Arc::new(5);

//     Command::repl(bot, move |a, b, c| {
//             let app = download_api.clone();
//             async move {
//                 answer(a, b, c, app).await
//             }
//         }
//     ).await;

// }

async fn answer(
    bot: Arc<Bot>,
    message: Message,
    command: Command,
    application: Arc<Mutex<Application>>,
) -> ResponseResult<()> {
    let message_str = {
        match command {
            Command::Help => {
                bot.send_message(message.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::List => {
                let searches = application.lock().await.list().await.unwrap();
                bot.send_message(
                    message.chat.id,
                    format!("{:?}", searches),
                )
                .await?
            }
            Command::Add { name, query } => {
                let _ = application.lock().await.add_search(name, query);
                bot.send_message(message.chat.id, "Add").await?
            }
        }
    };

    Ok(())
}

// async fn test_telegram_bot() {
//     env::set_var("TELOXIDE_TOKEN", "");

//     pretty_env_logger::init();
//     log::info!("Starting throw dice bot...");

//     let bot = Bot::new("");

//     teloxide::repl(bot, |bot: Bot, msg: Message| async move {
//         bot.send_dice(msg.chat.id).await?;
//         Ok(())
//     })
//     .await;
// }

// async fn test_scraper() -> Result<(), Box<dyn Error>> {
//     let download: DownloadAgent = Default::default();
//     let mut scraper = ScraperAgent::new(&download);

//     let results = scraper.run_query(
//         Search {
//             name: "Test".to_string().into(),
//             query: "Zelda Tears of the kingdom".to_string().into(),
//         }
//         .into(),
//     ).await?;

//     for result in results {
//         println!("{}", result)
//     }

//     Ok(())
// }
