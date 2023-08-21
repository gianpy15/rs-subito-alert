use std::{fmt::Display, rc::Rc, sync::Arc};

#[derive(Debug)]
pub struct ItemResult {
    name: String,
    uri: Rc<String>,
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
            uri: Rc::new(uri),
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
            uri: Rc::new(uri.to_string()),
            date: None,
            price: None,
            town: None,
            city: None,
            state: None,
        }
    }

    pub fn get_uri(&self) -> Rc<String> {
        Rc::clone(&self.uri)
    }
}

impl Display for ItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{}", self.uri)?;
        if let Some(date) = &self.date {
            writeln!(f, "{}", date)?;
        }
        if let Some(price) = self.price {
            writeln!(f, "{}€", price)?;
        }
        if let (Some(town), Some(city)) = (&self.town, &self.city) {
            writeln!(f, "{}{}", town, city)?;
        }
        if let Some(state) = &self.state {
            writeln!(f, "{}", state)?;
        }
        Ok(())
    }
}
