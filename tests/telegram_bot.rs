use std::error::Error;

use crate::test_doubles::application::ApplicationDouble;
use crate::test_doubles::serializer::SerializerDouble;
use rs_subito_alert::telegram_bot::telegram_bot_agent::TelegramBotAgent;
use rs_subito_alert::telegram_bot::telegram_bot_api::TelegramBotApi;

mod test_doubles;

#[test]
fn test_add_search() -> Result<(), Box<dyn Error>> {
    let mut application = ApplicationDouble::new();
    let mut serializer = SerializerDouble::new();
    let mut agent = TelegramBotAgent::new(&mut application);

    agent.add_search(String::from("Ciao"), String::from("Ciao"))?;

    assert_eq!(
        application.invocations,
        vec![Some((String::from("Ciao"), String::from("Ciao")))]
    );

    Ok(())
}

#[test]
fn test_list_searches() -> Result<(), Box<dyn Error>> {
    let mut application = ApplicationDouble::new();
    let mut serializer = SerializerDouble::new();
    let mut agent = TelegramBotAgent::new(&mut application);

    agent.list_searches()?;

    assert_eq!(application.invocations, vec![None]);

    Ok(())
}
