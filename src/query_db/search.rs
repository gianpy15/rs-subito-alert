use std::{fmt::Display, sync::Arc};

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

    pub fn name_as_string(&self) -> String {
        (Arc::clone(&self.name)).as_ref().clone()
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "➡️**{}**\n __{}__\n\n", self.name, self.query)
    }
}
