use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Search {
    pub name: Arc<str>,
    pub query: Arc<str>,
}

impl Search {
    pub fn new(name: &str, query: &str) -> Search {
        Search {
            name: Arc::from(name),
            query: Arc::from(query),
        }
    }

    pub fn name_as_str(&self) -> &str {
        &self.name
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<b>{}</b>", self.name)?;
        writeln!(f, "<i>{}</i>", self.query)?;
        Ok(())
    }
}
