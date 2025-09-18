use crate::search_providers::SearchProvider;
use crate::torrent::Torrent;

use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::Request;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use log::info;
use serde::Deserialize;

use std::error::Error;

// Not all fields are actually used, but we add all that are returned by the API.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub leechers: String,
    pub seeders: String,
    pub num_files: String,
    pub size: String,
    pub username: String,
    pub added: String,
    pub status: String,
    pub category: String,
    pub imdb: String,
}

pub struct PirateBaySearch {}

impl PirateBaySearch {
    pub fn new() -> PirateBaySearch {
        PirateBaySearch {}
    }
}

#[async_trait]
impl SearchProvider for PirateBaySearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on PirateBay");
        let url = format!("https://apibay.org/q.php?q={}", term);

        let https = HttpsConnector::new();
        let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

        let request = Request::get(url)
            .header(hyper::header::USER_AGENT, super::USER_AGENT)
            .body(Empty::new())
            .expect("Request builder");

        let mut res = client.request(request).await?;

        info!("Status: {}", res.status());
        let mut bytes = Vec::new();

        while let Some(frame) = res.body_mut().frame().await {
            let frame = frame?;
            if let Some(data) = frame.data_ref() {
                bytes.extend(data);
            }
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
            size_bytes: entry.size.parse().ok(),
            source: "PB",
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
            assert!(torrent.size_bytes.is_some());
        }
    }

    #[test]
    fn test_parse_piratebay_empty() {
        let torrents = super::parse_piratebay(TEST_DATA_EMPTY).unwrap();
        assert_eq!(torrents.len(), 0);
    }
}
