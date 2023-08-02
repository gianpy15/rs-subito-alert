
use std::{fmt::Display};

use isahc::{ReadResponseExt};
use regex::{Regex};
use soup::prelude::*;

#[derive(Debug)]
struct ItemResult {
    name: String,
    uri: String,
    price: Option<i32>,
    town: Option<String>,
    city: Option<String>,
    state: Option<String>
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

fn main() -> Result<(),  Box<dyn std::error::Error>>{
    let mut response = isahc::get("https://www.subito.it/annunci-italia/vendita/usato/?q=zelda%20tears%20of%20the%20kingdom")?;
    let body = response.text()?;

    let soup = Soup::new(&body);

    let product_list_items = soup.tag("div").class(Regex::new("item-key-data")?).find_all();

    for product in product_list_items {
        let parent_error = "Cannot get parent";
        let title = product.tag("h2").find().ok_or("Cannot collect title")?.text();
        let link = product.parent().ok_or(parent_error)?.parent().ok_or(parent_error)?.parent().ok_or(parent_error)?.parent().ok_or(parent_error)?.get("href").ok_or_else(|| "Cannot collect uri")?;

        let price=&product.tag("p").class(Regex::new("price")?).find().ok_or("Cannot get price block")?.children;
        let price_nodes = price.borrow();
        let mut price = price_nodes.get(0).and_then(|node| Some(node.text())).unwrap();
        price.truncate(price.len() - 5);
        // let price_num = price.borrow().tag("p").class(Regex::new("price")?).find().unwrap();

        let town = product.tag("span").class(Regex::new("town")?).find().and_then(|node| Some(node.text()));
        let city = product.tag("span").class(Regex::new("city")?).find().and_then(|node| Some(node.text()));

        let state = price_nodes.get(1).and_then(|node| Some(node.text()));
        
        let result = ItemResult {name: title, uri: link, price: Some(price.parse::<i32>().unwrap()), town, city, state};
        println!("{}", result);
    }
    Ok(())
}