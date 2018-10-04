//! Simple library for quickly fetching a list of URLs from a webpage.
//! 
//! # Example
//! ```rust,no_run
//! extern crate url_scraper;
//! use url_scraper::UrlScraper;
//! 
//! let scraper = UrlScraper::new("http://phoronix.com/").unwrap();
//! for (text, url) in scraper.into_iter() {
//!     println!("{}: {}", text, url);
//! }
//!```

extern crate failure;
extern crate reqwest;
extern crate scraper;

#[macro_use] extern crate failure_derive;

use reqwest::Url;
use scraper::{Html, html::Select, Selector};

/// Stores the HTML document in memory.
pub struct UrlScraper {
    url: Url,
    html: Html,
    selector: Selector,
}

impl UrlScraper {
    pub fn new(url: &str) -> Result<Self, Error> {
        let url = Url::parse(url)?;
        let mut resp = reqwest::get(url.clone())?;
        let html = resp.text()?;

        Ok(Self {
            url,
            html: Html::parse_document(&html),
            selector: Selector::parse("a").expect("failed to create <a> selector"),
        })
    }

    pub fn into_iter<'a>(&'a self) -> UrlIter<'a, 'a> {
        UrlIter {
            url: &self.url,
            data: self.html.select(&self.selector)
        }
    }
}

/// An Iterator that returns `(String, Url)` pairs per iteration.
pub struct UrlIter<'a, 'b> {
    url: &'a Url,
    data: Select<'a, 'b>
}

impl<'a, 'b> Iterator for UrlIter<'a, 'b> {
    type Item = (String, Url);

    fn next(&mut self) -> Option<Self::Item> {
        for element in &mut self.data {
            if let Some(url) = element.value().attr("href") {
                if ! url.starts_with("?") {
                    if let Ok(url) = self.url.join(url) {
                        return Some((element.inner_html(), url));
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to parse URL: {}", why)]
    UrlParsing { why: reqwest::UrlError },
    #[fail(display = "failure in request: {}", why)]
    Request { why: reqwest::Error }
}

impl From<reqwest::UrlError> for Error {
    fn from(why: reqwest::UrlError) -> Error {
        Error::UrlParsing { why }
    }
}

impl From<reqwest::Error> for Error {
    fn from(why: reqwest::Error) -> Error {
        Error::Request { why }
    }
}