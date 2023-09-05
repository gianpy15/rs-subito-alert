use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Search {
    pub name: Arc<str>,
    pub query: Arc<str>,
    price: Option<i32>,
}

impl Search {
    pub fn name_as_str(&self) -> &str {
        &self.name
    }

    pub fn min_price(&self) -> Option<i32> {
        self.price
    }

    pub fn new(name: &str, query: &str, price: Option<i32>) -> Self {
        Self {
            name: Arc::from(name),
            query: Arc::from(query),
            price: price,
        }
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<b>{}</b>", self.name)?;
        write!(f, "<i>{}</i>", self.query)?;
        if let Some(p) = self.price {
            write!(f, " → {}", p)?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}
