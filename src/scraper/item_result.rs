use std::fmt::Display;

#[derive(Debug)]
pub struct ItemResult {
    name: String,
    uri: String,
    price: Option<i32>,
    town: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

impl ItemResult {
    pub fn new(
        name: String,
        uri: String,
        price: Option<i32>,
        town: Option<String>,
        city: Option<String>,
        state: Option<String>,
    ) -> ItemResult {
        ItemResult {
            name,
            uri,
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
