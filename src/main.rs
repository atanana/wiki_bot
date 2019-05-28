extern crate regex;
extern crate reqwest;
extern crate select;

use std::error::Error;

use regex::{Captures, Regex};
use reqwest::Response;
use select::document::Document;
use select::predicate::Attr;

mod errors;

fn main() {
    test_print().unwrap();
}

fn test_print() -> Result<(), Box<Error>> {
    // let response = get_page()?;
    // let data = parse_data(response)?;
    let data: Vec<String> = vec![r#"<li><b><a href="/wiki/%D0%9C%D1%83%D0%BD%D0%BA,_%D0%9F%D0%B5%D1%82%D0%B5%D1%80_(%D0%BF%D1%80%D0%B5%D0%B4%D0%BF%D1%80%D0%B8%D0%BD%D0%B8%D0%BC%D0%B0%D1%82%D0%B5%D0%BB%D1%8C)" title="Мунк, Петер (предприниматель)">Еврейский бизнесмен</a></b> торговал <a href="/wiki/%D0%A0%D0%BE%D0%B6%D0%B4%D0%B5%D1%81%D1%82%D0%B2%D0%B5%D0%BD%D1%81%D0%BA%D0%B0%D1%8F_%D1%91%D0%BB%D0%BA%D0%B0" title="Рождественская ёлка">рождественскими ёлками</a> и подряжался развивать туризм в <a href="/wiki/%D0%AD%D0%BB%D1%8C-%D0%93%D0%B8%D0%B7%D0%B0" title="Эль-Гиза">Гизе</a>.</li>"#.to_string()];
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