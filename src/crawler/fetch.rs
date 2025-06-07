use reqwest::{Client};
use std::time::Duration;

lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .user_agent("open-crawler/0.1 (+https://yourdomain.com)")
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build reqwest client");
}

pub async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = CLIENT.get(url).send().await?;

    if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
        if !content_type.to_str()?.starts_with("text/html") {
            return Err("Non-HTML content".into());
        }
    }

    Ok(response.text().await?)
}