# url-scraper

Rust crate for scraping URLs from HTML pages.

## Example

```rust
extern crate url_scraper;
use url_scraper::UrlScraper;

fn main() {
    let directory = "http://phoronix.com/";

    let scraper = UrlScraper::new(directory).unwrap();
    for (text, url) in scraper.into_iter() {
        println!("{}: {}", text, url);
    }
}
```