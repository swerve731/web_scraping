use product_scraper::scrapers::ProductScraper;
use product_scraper::error::Result;
use product_scraper::scrapers;




#[tokio::main]
async fn main() -> Result<()> {
    let products = scrapers::stockx::StockxScraper::search_scrape("jordans".into(), 5).await?;

    println!("{:?}", products);
    Ok(())
}
