use reqwest::Client;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let metro_url = "https://www.metro.ca/epicerie-en-ligne/circulaire";
    let res = client.get(metro_url).send().await?;

    if !res.status().is_success() {
        panic!("Failed to get the Metro website");
    };

    let document = Html::parse_document(&res.text().await?);

    Ok(())
}

