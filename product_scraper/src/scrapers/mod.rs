pub mod goat;
pub mod stockx;
pub mod ebay;



pub trait Scraper {
    fn search_url(term: String) -> String;
}
// im using traits so later i can have a function like search_products(term) 
//and it will asynchronously get products from all the sites that have a product scraper
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
