pub mod bot_handlers {
    use std::sync::Arc;

    use teloxide::{
        dispatching::{
            dialogue::{self, InMemStorage},
            UpdateFilterExt, UpdateHandler,
        },
        dptree,
        prelude::Dialogue,
        requests::Requester,
        types::{Message, Update},
        utils::command::BotCommands,
        Bot,
    };
    use tokio::sync::Mutex;

    use crate::{
        application::{application_api::ApplicationApi, subito::Subito},
        notification::telegram_notifier::TelegramNotifier,
        query_db::query_engine::QueryEngine,
        scraper::{downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent},
        serializer::serializer_agent::SerializerAgent,
        telegram_bot::{commands::Command, state::State},
    };

    type MyDialogue = Dialogue<State, InMemStorage<State>>;
    type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
    type Application = Arc<
        Mutex<
            Subito<
                QueryEngine<SerializerAgent>,
                ScraperAgent<DownloadAgent>,
                TelegramNotifier<SerializerAgent>,
            >,
        >,
    >;

    pub async fn schema(
        application: Application,
    ) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
        use dptree::case;
        let query_app = Arc::clone(&application);
        let start_app = Arc::clone(&application);
        let list_app = Arc::clone(&application);

        let command_handler = teloxide::filter_command::<Command, _>()
            .branch(
                case![State::Idle]
                    .branch(case![Command::Help].endpoint(help))
                    .branch(case![Command::Add].endpoint(add)),
            )
            .branch(
                case![Command::List].endpoint(move |bot, dialogue, message| {
                    let app = Arc::clone(&list_app);
                    async move { list(bot, dialogue, message, app).await }
                }),
            )
            .branch(
                case![Command::Start].endpoint(move |bot, dialogue, message| {
                    let app = Arc::clone(&start_app);
                    async move { start(bot, dialogue, message, app).await }
                }),
            );
        //.branch(case![Command::Cancel].endpoint(cancel));

        let message_handler =
            Update::filter_message()
                .branch(command_handler)
                .branch(case![State::ReceiveSearchName].endpoint(receive_search_name))
                .branch(case![State::ReceiveSearchQuery { search_name }].endpoint(
                    move |bot, dialogue, search_name, callback| {
                        let app = Arc::clone(&query_app);
                        async move {
                            receive_query_name(bot, dialogue, search_name, callback, app).await
                        }
                    },
                ))
                .branch(dptree::endpoint(invalid_state));

        dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
    }

    async fn start(
        bot: Arc<Bot>,
        _dialogue: MyDialogue,
        message: Message,
        application: Application,
    ) -> HandlerResult {
        application
            .lock()
            .await
            .add_user(format!("{}", message.chat.id))
            .await
            .unwrap();
        bot.send_message(message.chat.id, "Welcome!").await?;
        Ok(())
    }

    async fn add(bot: Arc<Bot>, dialogue: MyDialogue, message: Message) -> HandlerResult {
        bot.send_message(message.chat.id, "Insert the name of the search.")
            .await?;
        dialogue.update(State::ReceiveSearchName).await?;
        Ok(())
    }

    async fn list(
        bot: Arc<Bot>,
        _dialogue: MyDialogue,
        message: Message,
        application: Application,
    ) -> HandlerResult {
        let searches = application.lock().await.list().await.unwrap();
        bot.send_message(message.chat.id, format!("{:?}", searches))
            .await?;
        Ok(())
    }

    async fn help(bot: Arc<Bot>, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;
        Ok(())
    }

    // async fn cancel(bot: Arc<Bot>, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    //     bot.send_message(msg.chat.id, "Cancelling the dialogue.")
    //         .await?;
    //     dialogue.exit().await?;
    //     Ok(())
    // }

    async fn invalid_state(bot: Arc<Bot>, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(
            msg.chat.id,
            "Unable to handle the message. Type /help to see the usage.",
        )
        .await?;
        dialogue.exit().await?;
        Ok(())
    }

    async fn receive_search_name(
        bot: Arc<Bot>,
        dialogue: MyDialogue,
        msg: Message,
    ) -> HandlerResult {
        match msg.text().map(ToOwned::to_owned) {
            Some(search_name) => {
                bot.send_message(msg.chat.id, "Insert the search query.")
                    .await?;
                dialogue
                    .update(State::ReceiveSearchQuery { search_name })
                    .await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Please, send me the name of the search.")
                    .await?;
            }
        }

        Ok(())
    }

    async fn receive_query_name(
        bot: Arc<Bot>,
        dialogue: MyDialogue,
        search_name: String, // Available from `State::ReceiveProductChoice`.
        message: Message,
        application: Application,
    ) -> HandlerResult {
        match message.text().map(ToOwned::to_owned) {
            Some(search_query) => {
                let _ = application
                    .lock()
                    .await
                    .add_search(search_name.clone(), search_query.clone())
                    .await;
                bot.send_message(
                    dialogue.chat_id(),
                    format!("Added {search_name} to the list of searches."),
                )
                .await?;
                dialogue.exit().await?;
            }
            None => {
                bot.send_message(message.chat.id, "Please, send me the query of the search.")
                    .await?;
            }
        }

        Ok(())
    }
}
