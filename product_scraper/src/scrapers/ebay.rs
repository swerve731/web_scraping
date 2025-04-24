use fantoccini::Locator;
use scraper::{Html, Selector};

use crate::default_client;

use super::{Product, ProductScraping, ScraperUtils};

pub struct EbayScraper;

impl ScraperUtils for EbayScraper {
    // this is the url for the search form on stockX
    fn search_url(term: String) -> String {
        format!("https://www.ebay.com/sch/i.html?_nkw={}", term)
    }
}

#[async_trait::async_trait]
impl ProductScraping for EbayScraper {
    async fn product_search_scrape(term: String, limit: usize) -> crate::Result<Vec<Product>> {
        let c = default_client().await?;

        c.goto(Self::search_url(term).as_str()).await?;

        let product_elements = c.find_all(Locator::Css(r#"li.s-item"#)).await?;
        
        let mut i = 0;
        let mut products = Vec::new();
        let mut limit = limit;
        while product_elements.len() > i && i < limit{
            let raw_element = product_elements[i].html(true).await?;
            let html_element = Html::parse_fragment(&raw_element);

            let title_selector = Selector::parse(r#"div.s-item__title"#)?;
            let title: String = html_element.select(&title_selector)
                .next()
                .ok_or(crate::error::Error::NotFound(format!("StockX title not found for element: {:?}", raw_element)))?
                .text()
                .collect::<String>();

            

            let price_selector = Selector::parse(r#"span.s-item__price"#)?;
            
            let price_string: String = html_element.select(&price_selector)
                .next()
                .ok_or(crate::error::Error::NotFound(format!("Ebay price not found for element: {:?}", raw_element)))?
                .text()
                .collect::<String>();


            let price_string =  if let Some(i) = price_string.find("to") {
                let s = price_string[0..i].into();
                s
            } else {
                price_string
            };


            let parsed_price = price_string
                .replace(['$', ',', '£', '€', ' '], "")
                .trim()
                .parse::<f64>()
                .map_err(|e| crate::error::Error::WrongDataType(format!("Could not parse the ebay price element: {e:?}")))?;
            
                // .parse()
                // .map_err(|e| crate::error::Error::WrongDataType(format!("Could not parse the ebay price element: {:?}\n\n Parsing Error: {:?}", price_string, e)))?;
            
            // I think ebay has some kind of hidden element since it returns Shop on Ebay 20$
            // this is to skip the element if it is the fake

            if title != "Shop on eBay"{
                products.push(Product::new(title, parsed_price));


            } else {
                // this doesnt count as a product
                // add one to the limit since i use indexing and if the limit is 70 it should return 70 valid elements
                limit+=1;
            }
            i+=1;

        };
    
        Ok(products)
    }
}