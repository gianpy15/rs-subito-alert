use std::fmt::Display;

#[derive(Debug)]
pub struct ItemResult {
    name: String,
    uri: String,
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
            uri,
            date,
            price,
            town,
            city,
            state,
        }
    }

    pub fn default(name: String, uri: String) -> ItemResult {
        ItemResult {
            name,
            uri,
            date: None,
            price: None,
            town: None,
            city: None,
            state: None,
        }
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
