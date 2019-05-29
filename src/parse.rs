use std::error::Error;

use regex::{Captures, Regex};
use reqwest::Response;
use select::document::Document;
use select::predicate::Attr;

use crate::errors::NoDyk;

pub fn parse_data(page: Response) -> Result<Vec<String>, Box<Error>> {
    let document = Document::from_read(page)?;
    let dyk_root = document.find(Attr("id", "main-dyk")).next().ok_or(NoDyk)?;
    let list = dyk_root.children()
        .filter(|element| element.name() == Some("ul"))
        .flat_map(|element| element.children())
        .filter(|element| element.name() == Some("li"));
    Ok(list.map(|element| element.html()).collect())
}

pub fn clear_data(data: Vec<String>) -> Result<Vec<String>, Box<Error>> {
    let tag_regex = Regex::new(r"</?(\w+).*?>")?;
    Ok(data.iter().map(|line| clear_line(line, &tag_regex)).collect())
}

fn clear_line(line: &String, tag_regex: &Regex) -> String {
    let line = line.replace("/wiki", "https://ru.wikipedia.org/wiki");
    tag_regex.replace_all(&line, |capture: &Captures| {
        match capture[1].as_ref() {
            "a" => capture[0].to_string(),
            _ => "".to_string()
        }
    }).to_string()
}