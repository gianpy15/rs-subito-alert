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
        vec![Arc::new(Search::new("Test", "test"))],
        vec![Arc::from("test")],
    )
}

#[tokio::test]
#[serial]
async fn test_path_is_correct() {
    let serializer: SerializerAgent = SerializerAgent::new("database.json", Some("test")).await;

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
    let serializer = SerializerAgent::new("database.json", Some("test")).await;

    serializer.serialize(&database).await?;

    let serialized_str = fs::read_to_string(serializer.get_full_path())?;

    assert_eq!(
        serialized_str,
        String::from(
            "{\"searches\":{\"Test\":{\"name\":\"Test\",\"query\":\"test\"}},\"items\":[\"test\"]}"
        )
    );

    <SerializerAgent as SerializerApi<DataBase>>::clear(&serializer).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_can_read_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = data_base();
    let serializer = SerializerAgent::new("database.json", Some("test")).await;

    serializer.serialize(&database).await?;
    let loaded_db = serializer.deserialize().await?;

    assert_eq!(database, loaded_db);
    <SerializerAgent as SerializerApi<DataBase>>::clear(&serializer).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_can_write_env() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new(String::from("api_key"));
    let serializer: SerializerAgent = SerializerAgent::new("telegram.json", Some("test")).await;

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
    <SerializerAgent as SerializerApi<DataBase>>::clear(&serializer).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_can_read_env() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new(String::from("api_key"));
    let serializer: SerializerAgent = SerializerAgent::new("telegram.json", Some("test")).await;

    serializer.serialize(&env).await?;
    let loaded_db = serializer.deserialize().await?;

    assert_eq!(env, loaded_db);
    <SerializerAgent as SerializerApi<DataBase>>::clear(&serializer).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_reset_application() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new("api_key".to_string());
    let database: DataBase = Default::default();
    let env_serializer: SerializerAgent = SerializerAgent::new("telegram.json", Some("test")).await;
    let db_serializer = SerializerAgent::new("database.json", Some("test")).await;
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("subito-alert/test");

    let _ = env_serializer.serialize(&env).await;
    let _ = db_serializer.serialize(&database).await;

    assert!(config_dir.exists());

    let _ = <SerializerAgent as SerializerApi<TelegramEnvironment>>::clear(&env_serializer).await;
    let _ = <SerializerAgent as SerializerApi<DataBase>>::clear(&db_serializer).await;

    assert!(!config_dir.exists());

    Ok(())
}
