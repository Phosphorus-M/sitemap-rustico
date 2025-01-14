use serde::{Deserialize, Serialize};
use super::url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "urlset")]
pub struct UrlSet {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    url: Vec<Url>,
}

impl UrlSet {
    pub fn new(url: Vec<Url>) -> Self {
        let xmlns = "http://www.sitemaps.org/schemas/sitemap/0.9".to_string();

        UrlSet {
            xmlns,
            url,
        }
    }
}