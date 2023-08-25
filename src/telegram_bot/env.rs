use std::{collections::HashSet, num::ParseIntError};

use serde::{Deserialize, Serialize};
use teloxide::types::ChatId;

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramEnvironment {
    api_key: String,
    chat_ids: HashSet<ChatId>,
}

impl TelegramEnvironment {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            chat_ids: HashSet::new(),
        }
    }

    pub fn get_token(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_chat_ids(&self) -> Vec<ChatId> {
        self.chat_ids.clone().into_iter().collect()
    }

    pub fn add_user(&mut self, id: String) -> Result<(), ParseIntError> {
        self.chat_ids.insert(ChatId(id.parse::<i64>()?));
        Ok(())
    }
}
