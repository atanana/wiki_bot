extern crate reqwest;
extern crate select;

use std::error::Error;
use std::fmt;

use reqwest::Response;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

fn main() {
    test_print().unwrap();
}

fn test_print() -> Result<(), Box<Error>> {
    let response = get_page()?;
    let data = parse_data(response);
    println!("{:#?}", data);
    Ok(())
}

fn get_page() -> reqwest::Result<Response> {
    reqwest::get("https://ru.wikipedia.org/wiki/%D0%97%D0%B0%D0%B3%D0%BB%D0%B0%D0%B2%D0%BD%D0%B0%D1%8F_%D1%81%D1%82%D1%80%D0%B0%D0%BD%D0%B8%D1%86%D0%B0")
}

fn parse_data(page: Response) -> Result<Vec<String>, Box<Error>> {
    let document = Document::from_read(page)?;
    let query = Attr("id", "main-dyk").descendant(Name("ul"));
    let dyk_root = document.find(Attr("id", "main-dyk")).next().ok_or(NoDyk)?;
    let list = dyk_root.children()
        .filter(|element| element.name() == Some("ul"))
        .flat_map(|element| element.children())
        .filter(|element| element.name() == Some("li"));
    let lines = list.map(|element| element.html()).collect::<Vec<_>>();
    Ok(lines)
}

#[derive(Debug, Clone)]
struct NoDyk;

impl fmt::Display for NoDyk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No dyk!")
    }
}

impl Error for NoDyk {}