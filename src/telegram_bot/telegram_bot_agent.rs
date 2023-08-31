use crate::application;
use crate::serializer::serializer_agent::SerializerAgent;
use crate::serializer::serializer_api::SerializerApi;
use crate::telegram_bot::commands::Command;
use crate::telegram_bot::handlers::bot_handlers;
use crate::telegram_bot::state::State;
use crate::types::Application;

use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;

use super::env::TelegramEnvironment;
use super::telegram_bot_api::TelegramBotApi;

pub struct TelegramBotAgent {
    env_serializer: SerializerAgent,
    bot: Arc<Bot>,
}

impl TelegramBotAgent {
    pub async fn new(env_serializer: SerializerAgent) -> Self {
        let env = env_serializer
            .deserialize()
            .await
            .ok()
            .unwrap_or(TelegramEnvironment::default());
        let bot = Arc::new(Bot::new(env.get_token()));
        Self {
            env_serializer,
            bot,
        }
    }
}

impl TelegramBotAgent {
    async fn schema(
        &self,
        application: Application,
    ) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
        use dptree::case;
        let query_app = Arc::clone(&application);
        let start_app = Arc::clone(&application);
        let list_app = Arc::clone(&application);
        let delete_dialogue_app = Arc::clone(&application);
        let delete_app = Arc::clone(&application);

        let command_handler = teloxide::filter_command::<Command, _>()
            .branch(
                case![State::Idle]
                    .branch(case![Command::Help].endpoint(bot_handlers::help))
                    .branch(case![Command::Add].endpoint(bot_handlers::add)),
            )
            .branch(
                case![Command::List].endpoint(move |bot, dialogue, message| {
                    let app = Arc::clone(&list_app);
                    async move { bot_handlers::list(bot, dialogue, message, app).await }
                }),
            )
            .branch(
                case![Command::Delete].endpoint(move |bot, dialogue, message| {
                    let app = Arc::clone(&delete_dialogue_app);
                    async move { bot_handlers::delete_dialogue(bot, dialogue, message, app).await }
                }),
            )
            .branch(
                case![Command::Start].endpoint(move |bot, dialogue, message| {
                    let app = Arc::clone(&start_app);
                    async move { bot_handlers::start(bot, dialogue, message, app).await }
                }),
            )
            .branch(case![Command::Cancel].endpoint(bot_handlers::cancel));

        let message_handler = Update::filter_message()
            .branch(command_handler)
            .branch(case![State::ReceiveSearchName].endpoint(bot_handlers::receive_search_name))
            .branch(case![State::ReceiveSearchQuery { search_name }].endpoint(
                move |bot, dialogue, search_name, callback| {
                    let app = Arc::clone(&query_app);
                    async move {
                        bot_handlers::receive_query_name(bot, dialogue, search_name, callback, app)
                            .await
                    }
                },
            ))
            .branch(dptree::endpoint(bot_handlers::invalid_state));
        let callback_query_handler = Update::filter_callback_query().branch(
            case![State::Delete].endpoint(move |bot, dialogue, callback| {
                let app = Arc::clone(&delete_app);
                async move { bot_handlers::delete(bot, dialogue, callback, app).await }
            }),
        );

        dialogue::enter::<Update, InMemStorage<State>, State, _>()
            .branch(message_handler)
            .branch(callback_query_handler)
    }
}

#[async_trait]
impl TelegramBotApi for TelegramBotAgent {
    async fn start(&self, application: Application) {
        println!("Application started");
        let _ = Dispatcher::builder(
            Arc::clone(&self.bot),
            self.schema(Arc::clone(&application)).await,
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    }

    async fn add_api_key(&self, api_key: &str) -> Result<(), Box<dyn Error>> {
        let mut env: TelegramEnvironment = self.env_serializer.deserialize().await?;
        env.set_token(api_key);
        self.env_serializer.serialize(&env).await?;
        Ok(())
    }

    fn get_bot(&self) -> Arc<Bot> {
        Arc::clone(&self.bot)
    }
}
