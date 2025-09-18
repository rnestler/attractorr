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
pub struct Response {
    pub torrents: Vec<Entry>,
    pub next: u64,
}

// Not all fields are actually used, but we add all that are returned by the API.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Entry {
    pub infohash: String,
    pub name: String,
    pub size_bytes: Option<u64>,
    pub created_unix: Option<u64>,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
    pub completed: Option<u32>,
    pub scraped_date: Option<i32>,
}

pub struct TorrentCsvSearch {}

impl TorrentCsvSearch {
    pub fn new() -> TorrentCsvSearch {
        TorrentCsvSearch {}
    }
}

#[async_trait]
impl SearchProvider for TorrentCsvSearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on Torrent-CSV");
        let url = format!(
            "https://torrents-csv.com/service/search?size=300&q={}",
            term
        );

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
        parse_torrent_csv(&body)
    }

    fn get_name(&self) -> &'static str {
        "TC"
    }
}

fn parse_torrent_csv(content: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
    let res: Response = serde_json::from_str(content)?;

    let results = res
        .torrents
        .iter()
        .map(|entry| Torrent {
            name: entry.name.clone(),
            magnet_link: format!("magnet:?xt=urn:btih:{}", entry.infohash),
            seeders: entry.seeders,
            leechers: entry.leechers,
            size_bytes: entry.size_bytes,
            source: "TC",
        })
        .collect();
    Ok(results)
}

#[cfg(test)]
mod test {
    static TEST_DATA: &str = include_str!("test_data/torrent-csv.json");
    static TEST_DATA_EMPTY: &str = include_str!("test_data/torrent-csv-empty.json");

    #[test]
    fn test_parse_torrent_csv() {
        let torrents = super::parse_torrent_csv(TEST_DATA).unwrap();
        assert_eq!(torrents.len(), 8);
        for torrent in torrents.iter() {
            assert!(torrent.magnet_link.starts_with("magnet:?"));
            assert!(torrent.seeders.is_some());
            assert!(torrent.leechers.is_some());
            assert!(torrent.size_bytes.is_some());
        }
    }

    #[test]
    fn test_parse_torrent_csv_empty() {
        let torrents = super::parse_torrent_csv(TEST_DATA_EMPTY).unwrap();
        assert_eq!(torrents.len(), 0);
    }
}
