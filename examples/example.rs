extern crate url_scraper;
use url_scraper::UrlScraper;

fn main() {
    let directory = "http://ppa.launchpad.net/freecad-maintainers/freecad-stable/ubuntu/pool/main/";

    let scraper = UrlScraper::new(directory).unwrap();
    for (text, url) in scraper.into_iter() {
        println!("{}: {}", text, url);
    }
}