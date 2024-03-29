extern crate regex;
extern crate reqwest;
extern crate select;

use std::env;
use std::error::Error;
use crate::hash::check_hashes;

mod hash;
mod parse;
mod errors;
mod io;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let is_debug = matches!(args.get(1).map(String::as_str), Some("-debug"));

    do_work(is_debug).await.unwrap()
}

async fn do_work(is_debug: bool) -> Result<(), Box<dyn Error>> {
    let response = io::get_page().await?;
    let data = parse::parse_data(response.as_ref())?;
    let hash = hash::calculate_hash(&data);
    let old_hash = hash::load_hash().unwrap_or(Vec::new());
    if check_hashes(&old_hash, &hash) {
        return Ok(());
    }
    send_report(is_debug, &data).await?;
    hash::save_hash(&hash)?;
    Ok(())
}

async fn send_report(is_debug: bool, data: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let clean_data = parse::clear_data(&data)?;
    if is_debug {
        for line in clean_data {
            println!("{}", line)
        }
    } else {
        io::send_data(clean_data).await?;
    }
    Ok(())
}