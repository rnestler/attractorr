use crate::search_providers::SearchProvider;
use crate::torrent::Torrent;

use async_trait::async_trait;
use hyper::Client;
use hyper_tls::HttpsConnector;
use log::info;
use serde::Deserialize;
use tokio::stream::StreamExt;

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
        let res = self.connection.get(url.parse().unwrap()).await?;

        info!("Status: {}", res.status());
        let mut body = res.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
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
}
