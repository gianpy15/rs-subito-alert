use std::{error::Error, fs, rc::Rc};

use rs_subito_alert::{query_db::{
    db::DataBase,
    search::Search,
}, serializer::{serializer_agent::SerializerAgent, serializer_api::SerializerApi}, telegram_bot::env::TelegramEnvironment};

fn data_base() -> DataBase {
    DataBase::new(
        vec![Rc::new(Search::new("Test".to_string(), "test".to_string()))],
        vec![Rc::new(String::from("test"))],
    )
}

#[test]
fn test_path_is_correct() {
    let serializer: SerializerAgent = Default::default();

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
            + "/subito-alert/database.json"
    );
}

#[test]
fn test_can_write_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = data_base();
    let mut serializer: SerializerAgent = Default::default();

    serializer.serialize(&database)?;

    let serialized_str = fs::read_to_string(serializer.get_full_path())?;

    assert_eq!(
        serialized_str,
        String::from(
            "{\"searches\":{\"Test\":{\"name\":\"Test\",\"query\":\"test\"}},\"items\":[\"test\"]}"
        )
    );
    Ok(())
}

#[test]
fn test_can_red_db() -> Result<(), Box<dyn Error>> {
    let database: DataBase = data_base();
    let mut serializer: SerializerAgent = Default::default();

    serializer.serialize(&database)?;
    let loaded_db = serializer.deserialize()?;

    assert_eq!(database, loaded_db);
    Ok(())
}

#[test]
fn test_can_write_env() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new(String::from("api_key"));
    let mut serializer: SerializerAgent = SerializerAgent::new(String::from("telegram.json"));

    serializer.serialize(&env)?;

    let serialized_str = fs::read_to_string(serializer.get_full_path())?;

    assert_eq!(
        serialized_str,
        String::from(
            "{\"api_key\":\"api_key\"}"
        )
    );
    Ok(())
}

#[test]
fn test_can_red_env() -> Result<(), Box<dyn Error>> {
    let env = TelegramEnvironment::new(String::from("api_key"));
    let mut serializer: SerializerAgent = SerializerAgent::new(String::from("telegram.json"));

    serializer.serialize(&env)?;
    let loaded_db = serializer.deserialize()?;

    assert_eq!(env, loaded_db);
    Ok(())
}