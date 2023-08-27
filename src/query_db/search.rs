use std::sync::Arc;

use serde::{Deserialize, Serialize};
use teloxide::requests::Requester;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Search {
    pub name: Arc<String>,
    pub query: Arc<String>,
}

impl Search {
    pub fn new(name: String, query: String) -> Search {
        Search {
            name: Arc::new(name),
            query: Arc::new(query),
        }
    }

    pub fn name_as_string(&self) -> String {
        (Arc::clone(&self.name)).as_ref().clone()
    }
}
