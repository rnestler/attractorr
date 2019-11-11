use crate::search_providers::SearchProvider;
use crate::torrent::Torrent;

use async_trait::async_trait;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use log::{error, info};
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

use std::error::Error;

pub struct KickassSearch {
    connection: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl KickassSearch {
    pub fn new() -> KickassSearch {
        let https = HttpsConnector::new().unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        KickassSearch { connection: client }
    }
}

#[async_trait]
impl SearchProvider for KickassSearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on Kickass");
        let url = format!("https://katcr.co/katsearch/page/1/{}", term);
        let request = Request::get(url)
            .header(hyper::header::USER_AGENT, super::USER_AGENT)
            .body(Body::empty())
            .expect("Request builder");
        let res = self.connection.request(request).await?;
        info!("Status: {}", res.status());

        let mut body = res.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
            let chunk = next?;
            bytes.extend(chunk);
        }

        let body = String::from_utf8(bytes)?;

        let document = Document::from(&*body);
        Ok(parse_kickass(&document))
    }

    fn get_name(&self) -> &'static str {
        "KAT"
    }
}

fn parse_kickass_entry(row: &Node<'_>) -> Result<Torrent, String> {
    let name = row
        .find(Class("torrents_table__torrent_title"))
        .nth(0)
        .ok_or_else(|| "Could not find 'torrents_table__torrent_title'".to_owned())
        .and_then(|n| Ok(n.text()))?;

    let link = row
        .find(Attr("title", "Torrent magnet link"))
        .nth(0)
        .ok_or_else(|| "Could not find magnet link".to_owned())?;

    let magnet_link = link
        .attr("href")
        .ok_or_else(|| "Could not find href element".to_owned())?;

    // table data is |Name|Size|Files|Age|Seeders|Leechers|
    let tds = row.find(Name("td"));
    let mut tds = tds.skip(4);
    let seeders = tds.next().and_then(|v| v.text().parse::<u32>().ok());
    let leechers = tds.next().and_then(|v| v.text().parse::<u32>().ok());

    let name = name.trim().to_owned();
    let magnet_link = magnet_link.replace(' ', "%20");
    Ok(Torrent {
        name,
        magnet_link,
        seeders,
        leechers,
    })
}

fn parse_kickass(document: &Document) -> Vec<Torrent> {
    let search_result = document.find(Attr("class", "tab_content")).into_selection();

    let mut result = vec![];
    // iterate table rows but skip header
    for table_row in search_result.find(Name("tr")).iter().skip(1) {
        match parse_kickass_entry(&table_row) {
            Ok(torrent) => result.push(torrent),
            Err(e) => error!("{}", e),
        }
    }
    result
}

#[cfg(test)]
mod test {
    use select::document::Document;
    static TEST_DATA: &str = include_str!("test_data/kickass.html");

    #[test]
    fn test_parse_kickass() {
        let document = Document::from(TEST_DATA);
        let torrents = super::parse_kickass(&document);
        assert_eq!(torrents.len(), 1);
        for torrent in torrents.iter() {
            assert!(torrent.magnet_link.starts_with("magnet:?"));
            assert!(torrent.seeders.is_some());
            assert!(torrent.leechers.is_some());
        }
    }
}
