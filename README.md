# Product Scraper

A web scraping tool built with Rust that fetches product information (title and price) from StockX and eBay based on a search term. It uses `fantoccini` for browser automation to handle dynamic content and `scraper` for parsing the HTML structure. Some other features that could be added is item streaming that returns products live as they are scraped and image grabbing.

## Features

*   Scrapes product search results from:
    *   StockX
    *   eBay
*   Extracts product title and price.
*   Runs searches concurrently for faster results.
*   Configured to bypass some common bot detection mechanisms.

## Prerequisites

Before you can run this project, make sure you have the following installed and running:

1.  **Rust:** The project is built with Rust. If you don't have it installed, you can get it via rustup.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **ChromeDriver:** This project uses `fantoccini` which requires a WebDriver instance.
    *   Download the ChromeDriver executable that matches your installed Google Chrome version from the official ChromeDriver website.
    *   Run ChromeDriver and ensure it's listening on port `4444`. Open a terminal and run:
        ```bash
        chromedriver --port=4444
        ```
        Keep this terminal window open while running the scraper.

## Installation

1.  Clone this repository (or navigate to the project directory if you already have it):
    ```bash
    # git clone https://github.com/swerve731/web_scraping/edit/main/README.md # If applicable
    cd product_scraper
    ```


To run the scraper, use the `cargo run` command followed by `--` and your desired search term.

```bash
cargo run -- <search_term>
```
