use std::{
    error::Error,
    fs,
    io::{Seek, SeekFrom},
    sync::Arc,
};

use rs_subito_alert::{
    query_db::{db::DataBase, search::Search},
    serializer::{serializer_agent::SerializerAgent, serializer_api::SerializerApi},
    telegram_bot::env::TelegramEnvironment,
};
use serial_test::serial;

fn data_base() -> DataBase {
    DataBase::new(
        vec![Arc::new(Search::new(
            "Test".to_string(),
            "test".to_string(),
        ))],
        vec![Arc::new(String::from("test"))],
    )
}

#[tokio::test]
#[serial]
async fn test_path_is_correct() {
    let serializer: SerializerAgent =
        SerializerAgent::new(String::from("database.json"), Some(String::from("test"))).await;

    assert_eq!(
        serializer
            .get_full_path()
            .into_os_string()
            .into_string()
            .unwrap(),
        dirs::config_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "/subito-alert/test/database.json"
    );
}

#[tokio::test]
#[serial]
async fn test_can_write_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = data_base();
    let mut serializer =
        SerializerAgent::new(String::from("database.json"), Some(String::from("test"))).await;

    serializer.serialize(&database).await?;

    let serialized_str = fs::read_to_string(serializer.get_full_path())?;

    assert_eq!(
        serialized_str,
        String::from(
            "{\"searches\":{\"Test\":{\"name\":\"Test\",\"query\":\"test\"}},\"items\":[\"test\"]}"
        )
    );
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_can_read_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = data_base();
    let mut serializer =
        SerializerAgent::new(String::from("database.json"), Some(String::from("test"))).await;

    serializer.serialize(&database).await?;
    let loaded_db = serializer.deserialize().await?;

    assert_eq!(database, loaded_db);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_can_write_env() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new(String::from("api_key"));
    let mut serializer: SerializerAgent =
        SerializerAgent::new(String::from("telegram.json"), Some(String::from("test"))).await;

    serializer.serialize(&env).await?;

    let mut file_p = fs::File::open(serializer.get_full_path())?;
    file_p.seek(SeekFrom::Start(0))?;
    let serialized_str = fs::read_to_string(serializer.get_full_path())?;
    println!(
        "{}",
        serializer
            .get_full_path()
            .into_os_string()
            .into_string()
            .ok()
            .unwrap()
    );

    assert_eq!(
        serialized_str,
        String::from("{\"api_key\":\"api_key\",\"chat_ids\":[]}")
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_can_read_env() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new(String::from("api_key"));
    let mut serializer: SerializerAgent =
        SerializerAgent::new(String::from("telegram.json"), Some(String::from("test"))).await;

    serializer.serialize(&env).await?;
    let loaded_db = serializer.deserialize().await?;

    assert_eq!(env, loaded_db);

    Ok(())
}
