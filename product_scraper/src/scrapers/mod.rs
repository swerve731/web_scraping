pub mod goat;
pub mod stockx;
pub mod ebay;

use crate::error::Error;


pub trait Scraper {
    fn search_url(term: String) -> String;
}

#[async_trait::async_trait]
pub trait ProductScraper {
    async fn search_scrape(term: String, limit: usize) -> crate::Result<Vec<Product>>;
}

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product {
            name: name,
            price: price,
        }
    }
}
