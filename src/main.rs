extern crate regex;
extern crate reqwest;
extern crate select;

use std::error::Error;

use regex::{Captures, Regex};
use reqwest::{Client, Response};
use select::document::Document;
use select::predicate::Attr;

mod errors;

fn main() {
    test_print().unwrap();
}

fn test_print() -> Result<(), Box<Error>> {
    let response = get_page()?;
    let data = parse_data(response)?;
    let clean_data = clear_data(data)?;
    let result = send_data(clean_data)?;
    println!("{:#?}", result);
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

fn clear_data(data: Vec<String>) -> Result<Vec<String>, Box<Error>> {
    let tag_regex = Regex::new(r"</?(\w+).*?>")?;
    Ok(data.iter().map(|line| clear_line(line, &tag_regex)).collect())
}

fn clear_line(line: &String, tag_regex: &Regex) -> String {
    let line = line.replace("/wiki", "https://ru.wikipedia.org/wiki");
    tag_regex.replace_all(&line, |capture: &Captures| {
        match capture[1].as_ref() {
            "a" | "b" | "i" => capture[0].to_string(),
            _ => "".to_string()
        }
    }).to_string()
}

fn send_data(data: Vec<String>) -> reqwest::Result<Response> {
    let client = Client::new();
    let token = env!("BOT_TOKEN");
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let params = [
        ("chat_id", "@wiki_dyk"),
        ("text", &data.join("\n\n")),
        ("parse_mode", "HTML")
    ];
    let request = client.post(&url).form(&params);
    println!("{:#?}", request);
    request.send()
}