mod scraper;

use scraper::{MetroScraper, Scraper};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let metro_scraper = MetroScraper::new();
    metro_scraper.scrape_products().await?;

    Ok(())
}

