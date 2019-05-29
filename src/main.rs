extern crate regex;
extern crate reqwest;
extern crate select;

use std::error::Error;

use reqwest::{Client, Response};

mod hash;
mod parse;
mod errors;

fn main() {
    do_work().unwrap();
}

fn do_work() -> Result<(), Box<Error>> {
    let response = get_page()?;
    let data = parse::parse_data(response)?;
    let hash = hash::calculate_hash(&data);
    let old_hash = hash::load_hash();
    if hash == old_hash {
        return Ok(());
    }
    let clean_data = parse::clear_data(data)?;
    send_data(clean_data)?;
    hash::save_hash(hash)?;
    Ok(())
}

fn get_page() -> reqwest::Result<Response> {
    reqwest::get("https://ru.wikipedia.org/")
}

fn send_data(data: Vec<String>) -> reqwest::Result<Response> {
    let client = Client::new();
    let token = env!("BOT_TOKEN");
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let params = [
        ("chat_id", "@wiki_dyk"),
        ("text", &data.join("\n\n")),
        ("parse_mode", "HTML"),
        ("disable_web_page_preview", "true")
    ];
    client.post(&url).form(&params).send()
}