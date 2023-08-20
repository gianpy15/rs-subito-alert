use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramEnvironment {
    api_key: String,
}

impl TelegramEnvironment {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn get_token(&self) -> String {
        self.api_key.clone()
    }
}
