use std::error::Error;
pub trait TelegramBotApi {
    fn add_search(&mut self, name: String, query: String) -> Result<(), Box<dyn Error>>;
    fn list_searches(&mut self) -> Result<(), Box<dyn Error>>;
}
