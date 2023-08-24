use std::sync::Arc;

use serde::{Deserialize, Serialize};

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
}
