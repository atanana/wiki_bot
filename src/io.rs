use reqwest::{Client, Response};

pub async fn get_page() -> reqwest::Result<String> {
    let response = reqwest::get("https://ru.wikipedia.org/").await?;
    response.text().await
}

pub async fn send_data(data: Vec<String>) -> reqwest::Result<Response> {
    let client = Client::new();
    let token = env!("BOT_TOKEN");
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let params = [
        ("chat_id", "@wiki_dyk"),
        ("text", &data.join("\n\n")),
        ("parse_mode", "HTML"),
        ("disable_web_page_preview", "true")
    ];
    client.post(&url).form(&params).send().await
}