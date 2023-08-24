use serde::{Deserialize, Serialize};
use teloxide::types::ChatId;

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramEnvironment {
    api_key: String,
    chat_ids: Vec<ChatId>,
}

impl TelegramEnvironment {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            chat_ids: vec![],
        }
    }

    pub fn get_token(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_chat_ids(&self) -> Vec<ChatId> {
        self.chat_ids.clone()
    }
}
