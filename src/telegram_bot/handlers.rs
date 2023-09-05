pub mod bot_handlers {
    use std::{str::FromStr, sync::Arc};

    use teloxide::{
        adaptors::DefaultParseMode,
        dispatching::dialogue::InMemStorage,
        payloads::SendMessageSetters,
        prelude::Dialogue,
        requests::Requester,
        types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
        utils::command::BotCommands,
        Bot,
    };

    use crate::{
        application::application_api::ApplicationApi,
        telegram_bot::{commands::Command, state::State},
        types::Application,
    };

    type MyDialogue = Dialogue<State, InMemStorage<State>>;
    type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

    pub async fn start(
        bot: Arc<DefaultParseMode<Bot>>,
        _dialogue: MyDialogue,
        message: Message,
        application: Application,
    ) -> HandlerResult {
        application
            .lock()
            .await
            .add_user(message.chat.id.to_string().as_str())
            .await
            .unwrap();
        bot.send_message(message.chat.id, "Welcome\\!").await?;
        Ok(())
    }

    pub async fn add(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        message: Message,
    ) -> HandlerResult {
        bot.send_message(message.chat.id, "Insert the name of the search.")
            .await?;
        dialogue.update(State::ReceiveSearchName).await?;
        Ok(())
    }

    pub async fn delete_dialogue(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        message: Message,
        application: Application,
    ) -> HandlerResult {
        let searches = application.lock().await.list().await.unwrap();
        let searches_keyboard: Vec<Vec<InlineKeyboardButton>> = searches
            .into_iter()
            .map(|s| s.name_as_str().to_owned())
            .map(|search| InlineKeyboardButton::callback(search.clone(), search))
            .collect::<Vec<InlineKeyboardButton>>()
            .chunks(4)
            .map(|s| s.into())
            .collect();
        bot.send_message(message.chat.id, "Select a search to delete.")
            .reply_markup(InlineKeyboardMarkup::new(searches_keyboard))
            .await?;
        dialogue.update(State::Delete).await?;

        Ok(())
    }

    pub async fn delete(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        q: CallbackQuery,
        application: Application,
    ) -> HandlerResult {
        if let Some(search) = &q.data {
            let _ = application.lock().await.delete_search(search).await;
            bot.send_message(dialogue.chat_id(), format!("{search} deleted"))
                .await?;
            dialogue.exit().await?;
        }

        Ok(())
    }

    pub async fn list(
        bot: Arc<DefaultParseMode<Bot>>,
        _dialogue: MyDialogue,
        message: Message,
        application: Application,
    ) -> HandlerResult {
        let searches = application
            .lock()
            .await
            .list()
            .await
            .unwrap()
            .iter()
            .map(|item| item.to_string())
            .reduce(|cur, next| cur + "\n" + &next);
        bot.send_message(message.chat.id, searches.unwrap_or("".to_string()))
            .await?;
        Ok(())
    }

    pub async fn help(bot: Arc<DefaultParseMode<Bot>>, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;
        Ok(())
    }

    pub async fn cancel(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        msg: Message,
    ) -> HandlerResult {
        bot.send_message(msg.chat.id, "Cancelling the dialogue.")
            .await?;
        dialogue.exit().await?;
        Ok(())
    }

    pub async fn invalid_state(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        msg: Message,
    ) -> HandlerResult {
        bot.send_message(
            msg.chat.id,
            "Unable to handle the message. Type /help to see the usage.",
        )
        .await?;
        dialogue.exit().await?;
        Ok(())
    }

    pub async fn receive_search_name(
        bot: Arc<DefaultParseMode<Bot>>,
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

    pub async fn receive_query_name(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        search_name: String,
        message: Message,
    ) -> HandlerResult {
        match message.text().map(ToOwned::to_owned) {
            Some(search_query) => {
                bot.send_message(
                    message.chat.id,
                    "Insert the search price (0 to skip filter).",
                )
                .await?;
                dialogue
                    .update(State::ReceiveSearchPrice {
                        search_name,
                        search_query,
                    })
                    .await?;
            }
            None => {
                bot.send_message(message.chat.id, "Please, send me the name of the search.")
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn receive_query_price(
        bot: Arc<DefaultParseMode<Bot>>,
        dialogue: MyDialogue,
        (search_name, search_query): (String, String),
        message: Message,
        application: Application,
    ) -> HandlerResult {
        match message.text() {
            Some(search_price) => {
                let _ = application
                    .lock()
                    .await
                    .add_search(
                        &search_name,
                        &search_query,
                        FromStr::from_str(search_price).ok(),
                    )
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
