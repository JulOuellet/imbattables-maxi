use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};

use super::scraper::Scraper;

pub struct MetroScraper {
    client: Client,
    base_url: String
}

impl MetroScraper {
    pub fn new() -> Self {
        MetroScraper {
            client: Client::new(),
            base_url: "https://www.metro.ca/epicerie-en-ligne/circulaire".to_string()
        }
    }
}

#[async_trait]
impl Scraper for MetroScraper {

    async fn fetch_page(&self, page_url: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let res = self.client.get(page_url).send().await?;
        if !res.status().is_success() {
            panic!("Failed to fetch the Metro page.");
        }

        let document = Html::parse_document(&res.text().await?);
        Ok(document)
    }

    async fn scrape_products(&self) -> Result<(), Box<dyn std::error::Error>> {
        let document = self.fetch_page(&self.base_url).await?;
        let test_selector = Selector::parse("div.default-product-tile").unwrap();

        for element in document.select(&test_selector) {
            println!("{:?}", element);
        }

        Ok(())
    }

}

