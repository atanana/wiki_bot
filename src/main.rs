extern crate regex;
extern crate reqwest;
extern crate select;

use std::error::Error;
use crate::errors::NoDyk;

mod hash;
mod parse;
mod errors;
mod io;

fn main() {
    do_work().unwrap();
}

fn do_work() -> Result<(), Box<dyn Error>> {
    let response = io::get_page()?;
    let data = parse::parse_data(response)?;
    let hash = hash::calculate_hash(&data)?;
    let old_hash = hash::load_hash();
    if hash == &old_hash {
        return Ok(());
    }
    let clean_data = parse::clear_data(&data)?;
    io::send_data(clean_data)?;
    hash::save_hash(&hash)?;
    Ok(())
}