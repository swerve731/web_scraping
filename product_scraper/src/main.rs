use product_scraper::scrapers::ProductScraping;
use product_scraper::error::Result;
use product_scraper::scrapers;




#[tokio::main]
async fn main() -> Result<()> {
    

    // let products = scrapers::stockx::StockxScraper::product_search_scrape("jordans".into(), 30).await?;
    let products = scrapers::ebay::EbayScraper::product_search_scrape("jordans".into(), 100).await?;


    println!("{:?}", products);
    println!("Returned {} products from ebay", products.len());

    Ok(())
}
