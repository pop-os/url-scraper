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

extern crate reqwest;
extern crate scraper;

use reqwest::{Client, Url};
use scraper::{Html, html::Select, Selector};
use std::fmt;

/// Stores the HTML document in memory.
pub struct UrlScraper {
    url: Url,
    html: Html,
    selector: Selector,
}

impl UrlScraper {
    /// Constructs a new scraper from a given URL.
    pub fn new(url: &str) -> Result<Self, Error> {
        let client = Client::new();
        Self::new_with_client(url, &client)
    }

    /// Use an existing `reqwest::Client` to make a request.
    pub fn new_with_client(url: &str, client: &Client) -> Result<Self, Error> {
        let url = Url::parse(url)?;
        let mut resp = client.get(url.clone()).send()?;
        let html = resp.text()?;

        Ok(Self {
            url,
            html: Html::parse_document(&html),
            selector: Selector::parse("a").expect("failed to create <a> selector"),
        })
    }

    /// In case the HTML has already been fetched in advance, this can be used to parse from it directly.
    pub fn new_with_html(url: &str, html: &str) -> Result<Self, Error> {
        Ok(Self {
            url: Url::parse(url)?,
            html: Html::parse_document(html),
            selector: Selector::parse("a").expect("failed to create <a> selector"),
        })
    }

    /// Fetch the URLs using an iterator.
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

#[derive(Debug)]
pub enum Error {
    UrlParsing { why: reqwest::UrlError },
    Request { why: reqwest::Error }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match *self {
            Error::UrlParsing { ref why } => format!("failed to parse URL: {}", why),
            Error::Request { ref why } => format!("failure in request: {}", why),
        };
        f.write_str(&error)
    }
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