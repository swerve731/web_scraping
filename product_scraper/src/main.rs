use std::path::Display;

use product_scraper::scrapers::ProductScraping;
use futures::join;
use product_scraper::error::Result;
use product_scraper::scrapers;



#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let search_term = if args.len() > 1 {
        args[1].clone()
    } else {
        "jordans".to_string()
    };
    
    println!("Searching for: {}", search_term);

    
    let sx = scrapers::stockx::StockxScraper::product_search_scrape("jordans".into(), 10);
    let eb = scrapers::ebay::EbayScraper::product_search_scrape("jordans".into(), 10);

    // i can change this so it streams the prouduct items 
    let (stockx_result, ebay_result) = join!(sx, eb); // runs them at the same time without blocking

    let sx = stockx_result?;
    let eb = ebay_result?;
    println!("STOCKX {:?}", sx);
    println!("\n\nEBAY: {:?}", eb);

    println!("\n\nStockX returned {} products", sx.len());
    println!("Ebay returned {} products", eb.len());

    Ok(())

}