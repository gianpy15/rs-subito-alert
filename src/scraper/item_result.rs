use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct ItemResult {
    name: String,
    uri: Arc<String>,
    date: Option<String>,
    price: Option<i32>,
    town: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

impl ItemResult {
    pub fn new(
        name: String,
        uri: String,
        date: Option<String>,
        price: Option<i32>,
        town: Option<String>,
        city: Option<String>,
        state: Option<String>,
    ) -> ItemResult {
        ItemResult {
            name,
            uri: Arc::new(uri),
            date,
            price,
            town,
            city,
            state,
        }
    }

    pub fn default(name: &str, uri: &str) -> ItemResult {
        ItemResult {
            name: name.to_string(),
            uri: Arc::new(uri.to_string()),
            date: None,
            price: None,
            town: None,
            city: None,
            state: None,
        }
    }

    pub fn get_uri(&self) -> Arc<String> {
        Arc::clone(&self.uri)
    }
}

impl Display for ItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "**{}**", self.name)?;
        writeln!(f, "{}", self.uri)?;
        if let Some(date) = &self.date {
            writeln!(f, "🕑 {}", date)?;
        }
        if let Some(price) = self.price {
            writeln!(f, "💸 {}€", price)?;
        }
        if let (Some(town), Some(city)) = (&self.town, &self.city) {
            writeln!(f, "📍 {}{}", town, city)?;
        }
        if let Some(state) = &self.state {
            match state.as_str() {
                "Spedizione disponibile" => {
                    writeln!(f, "🚛 Disponibile")?;
                }

                _ => {
                    writeln!(f, "{}", state)?;
                }
            }
        }
        Ok(())
    }
}
