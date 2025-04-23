use fantoccini::Locator;
use scraper::{html, selector, Html, Selector};

use crate::default_client;

use super::{Product, ProductScraper, Scraper};

use crate::error::Error;


pub struct StockxScraper;

impl Scraper for StockxScraper {
    fn search_url(term: String) -> String {
        format!("https://stockx.com/search?s={}", term)
    }
}

#[async_trait::async_trait]
impl ProductScraper for StockxScraper {
    async fn search_scrape(term: String, limit: usize) -> crate::Result<Vec<Product>> {
        let c = default_client().await?;

        c.goto(Self::search_url(term).as_str()).await?;
        let product_elements = c.find_all(Locator::Css(r#"div[data-testid="productTile"]"#)).await?;
        
        let mut i = 0;

        let mut products = Vec::new();
        while product_elements.iter().next().is_some() && i < limit{
            let raw_element = product_elements[i].html(true).await?;
            let html_element = Html::parse_fragment(&raw_element);

            let title_selector = Selector::parse(r#"p[data-testid="product-tile-title"]"#)?;
            let title: String = html_element.clone().select(&title_selector)
                .next()
                .ok_or(crate::error::Error::NotFound(format!("StockX title not found for element: {:?}", raw_element)))?
                .text()
                .collect::<String>();

            println!("{}", title);
            
            let price_selector = Selector::parse(r#"p[data-testid="product-tile-lowest-ask-amount"]"#)?;
            
            let price_string: String = html_element.select(&price_selector)
                .next()
                .ok_or(crate::error::Error::NotFound(format!("StockX price not found for element: {:?}", raw_element)))?
                .text()
                .collect::<String>();

            let parsed_price: f64 = price_string
                .replace("$", "")
                .parse()
                .unwrap();
            println!("{}", parsed_price);


            products.push(Product::new(title, parsed_price));
            i+=1;
        };

    
        Ok(products)
    }
}