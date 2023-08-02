
use std::rc::Rc;

use isahc::{ReadResponseExt};
use regex::{Regex, Error};
use soup::prelude::*;
use rs_subito_alert::subito;

struct ItemResult {
    name: String,
    uri: String,
    price: String,
}

fn main() -> Result<(),  Box<dyn std::error::Error>>{
    let mut response = isahc::get("https://www.subito.it/annunci-italia/vendita/usato/?q=zelda%20tears%20of%20the%20kingdom")?;
    let body = response.text()?;

    let soup = Soup::new(&body);

    let product_list_items = soup.tag("div").class(Regex::new("item-key-data")?).find_all();
    let mut i = 0;

    for product in product_list_items {
        let title = product.tag("h2").find().unwrap();
        let link = product.parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().get("href").unwrap();

        let price=&product.tag("p").class(Regex::new("price")?).find().unwrap().children;
        let price_nodes = price.borrow();
        let price_node = price_nodes.get(0).unwrap();
        // let price_num = price.borrow().tag("p").class(Regex::new("price")?).find().unwrap();

        println!("{}", title.text());
        println!("{}", link);
        println!("{}", price_node.display());
        i+=1;
        if i == 2{
            break;
        }
    }
    Ok(())
}