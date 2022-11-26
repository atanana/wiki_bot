use std::error::Error;
use once_cell::sync::Lazy;

use regex::{Captures, Regex};
use select::document::Document;
use select::predicate::Attr;

use crate::errors::NoDyk;

pub fn parse_data(page: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let document = Document::from(page);
    let dyk_root = document.find(Attr("id", "main-dyk")).next().ok_or(NoDyk)?;
    let list = dyk_root.children()
        .filter(|element| element.name() == Some("ul"))
        .flat_map(|element| element.children())
        .filter(|element| element.name() == Some("li"));
    Ok(list.map(|element| element.html()).collect())
}

static TAG_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r"</?(\w+).*?>").unwrap() });

pub fn clear_data(data: &Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let cleared_data = data.iter()
        .map(|line| clear_line(line))
        .collect();
    Ok(cleared_data)
}

fn clear_line(line: &String) -> String {
    let line = line.replace("/wiki", "https://ru.wikipedia.org/wiki")
        .replace("&nbsp;", " ");
    TAG_REGEX.replace_all(&line, |capture: &Captures| {
        match capture[1].as_ref() {
            "a" => capture[0].to_string(),
            _ => "".to_string()
        }
    }).to_string()
}