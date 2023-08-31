use std::{error::Error, sync::Arc};

use rs_subito_alert::{
    notification::{notification_api::NotificationApi, telegram_notifier::TelegramNotifier},
    serializer::{serializer_agent::SerializerAgent, serializer_api::SerializerApi},
    telegram_bot::env::TelegramEnvironment,
};
use teloxide::Bot;

#[tokio::test]
async fn test_add_user() -> Result<(), Box<dyn Error>> {
    let env_serializer = Arc::new(SerializerAgent::new("telegram.json", Some("test")).await);
    let bot = Arc::new(Bot::new(""));
    let notifier = TelegramNotifier::new(Arc::clone(&env_serializer), bot);
    let _ = env_serializer
        .serialize(&TelegramEnvironment::new("api_key".to_string()))
        .await;

    notifier.add_user("1234").await?;

    let env: TelegramEnvironment = env_serializer.deserialize().await?;

    assert_eq!(
        env.get_chat_ids()
            .iter()
            .map(|id| format!("{id}"))
            .collect::<Vec<String>>(),
        vec!["1234"]
    );
    Ok(())
}
