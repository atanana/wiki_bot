extern crate reqwest;
extern crate select;

use std::error::Error;

use reqwest::Response;
use select::document::Document;
use select::predicate::{Attr};

mod errors;

fn main() {
    test_print().unwrap();
}

fn test_print() -> Result<(), Box<Error>> {
    let response = get_page()?;
    let data = parse_data(response)?;
    let clean_data = clear_data(data);
    println!("{:#?}", clean_data);
    Ok(())
}

fn get_page() -> reqwest::Result<Response> {
    reqwest::get("https://ru.wikipedia.org/")
}

fn parse_data(page: Response) -> Result<Vec<String>, Box<Error>> {
    let document = Document::from_read(page)?;
    let dyk_root = document.find(Attr("id", "main-dyk")).next().ok_or(errors::NoDyk)?;
    let list = dyk_root.children()
        .filter(|element| element.name() == Some("ul"))
        .flat_map(|element| element.children())
        .filter(|element| element.name() == Some("li"));
    Ok(list.map(|element| element.html()).collect())
}

fn clear_data(data: Vec<String>) -> Vec<String> {
    data.iter().map(|line| clear_line(line)).collect()
}

fn clear_line(line: &String) -> String {
    line.to_owned()
}