use async_trait::async_trait;
use scraper::Html;

#[async_trait]
pub trait Scraper {
    async fn fetch_page(&self, page_url: &str) -> Result<Html, Box<dyn std::error::Error>>;
    async fn scrape_products(&self) -> Result<(), Box<dyn std::error::Error>>;
}

