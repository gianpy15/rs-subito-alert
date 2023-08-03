use std::{error::Error, fmt::Display, fs, path::Path, process::exit};

use isahc::ReadResponseExt;
use regex::Regex;
use soup::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    Path::new("resources/example_page.html");
    let html = fs::read_to_string("address.txt")?;
    println!("{}", html);
    Ok(())
}
