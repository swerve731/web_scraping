use fantoccini::{ClientBuilder, Locator};
use product_scraper::scrapers::ProductScraper;
use serde_json::json;
use product_scraper::error::Result;
use product_scraper::{default_client, scrapers};




#[tokio::main]
async fn main() -> Result<()> {
    let products = scrapers::stockx::StockxScraper::search_scrape("jordans".into(), 5).await?;

    println!("{:?}", products);
    Ok(())
}
