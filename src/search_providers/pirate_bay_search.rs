use crate::search_providers::SearchProvider;
use crate::torrent::Torrent;

use async_trait::async_trait;
use hyper::{body::HttpBody, Body, Client, Request};
use hyper_tls::HttpsConnector;
use log::info;
use serde::Deserialize;

use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Entry {
    id: String,
    name: String,
    info_hash: String,
    leechers: String,
    seeders: String,
    num_files: String,
    size: String,
    username: String,
    added: String,
    status: String,
    category: String,
    imdb: String,
}

pub struct PirateBaySearch {
    connection: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl PirateBaySearch {
    pub fn new() -> PirateBaySearch {
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        PirateBaySearch { connection: client }
    }
}

#[async_trait]
impl SearchProvider for PirateBaySearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on PirateBay");
        let url = format!("https://apibay.org/q.php?q={}", term);

        let request = Request::get(url)
            .header(hyper::header::USER_AGENT, super::USER_AGENT)
            .body(Body::empty())
            .expect("Request builder");

        let mut res = self.connection.request(request).await?;

        info!("Status: {}", res.status());
        let mut bytes = Vec::new();
        while let Some(next) = res.data().await {
            let chunk = next?;
            bytes.extend(chunk);
        }

        let body = String::from_utf8(bytes)?;
        parse_piratebay(&body)
    }

    fn get_name(&self) -> &'static str {
        "PB"
    }
}

fn parse_piratebay(content: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
    let entries: Vec<Entry> = serde_json::from_str(content)?;

    let results = entries
        .iter()
        .filter(|entry| {
            entry.id != "0"
                && entry.name != "No results returned"
                && entry.info_hash != "0000000000000000000000000000000000000000"
        })
        .map(|entry| Torrent {
            name: entry.name.clone(),
            magnet_link: format!("magnet:?xt=urn:btih:{}", entry.info_hash),
            seeders: entry.seeders.parse().ok(),
            leechers: entry.leechers.parse().ok(),
        })
        .collect();
    Ok(results)
}

#[cfg(test)]
mod test {
    static TEST_DATA: &str = include_str!("test_data/piratebay.json");
    static TEST_DATA_EMPTY: &str = include_str!("test_data/piratebay-empty.json");

    #[test]
    fn test_parse_piratebay() {
        let torrents = super::parse_piratebay(TEST_DATA).unwrap();
        assert_eq!(torrents.len(), 100);
        for torrent in torrents.iter() {
            assert!(torrent.magnet_link.starts_with("magnet:?"));
            assert!(torrent.seeders.is_some());
            assert!(torrent.leechers.is_some());
        }
    }

    #[test]
    fn test_parse_piratebay_empty() {
        let torrents = super::parse_piratebay(TEST_DATA_EMPTY).unwrap();
        assert_eq!(torrents.len(), 0);
    }
}
