use crate::search_providers::SearchProvider;
use crate::torrent::Torrent;

use async_trait::async_trait;
use hyper::Client;
use hyper_tls::HttpsConnector;
use log::{error, info};
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

use std::error::Error;

pub struct PirateBaySearch {
    connection: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl PirateBaySearch {
    pub fn new() -> PirateBaySearch {
        let https = HttpsConnector::new().unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        PirateBaySearch { connection: client }
    }
}

#[async_trait]
impl SearchProvider for PirateBaySearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on PirateBay");
        let url = format!("https://thepiratebay.org/search/{}/0/99/0", term);
        let res = self.connection.get(url.parse().unwrap()).await?;

        info!("Status: {}", res.status());
        let mut body = res.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
            let chunk = next?;
            bytes.extend(chunk);
        }

        let body = String::from_utf8(bytes)?;

        let document = Document::from(&*body);
        Ok(parse_piratebay(&document))
    }

    fn get_name(&self) -> &'static str {
        "PB"
    }
}

fn parse_piratebay_entry(row: &Node<'_>) -> Result<Torrent, String> {
    let name = row
        .find(Class("detLink"))
        .nth(0)
        .ok_or_else(|| "Could not find 'detLink'".to_owned())
        .and_then(|n| Ok(n.text()))?;

    let link = row
        .find(Attr("title", "Download this torrent using magnet"))
        .nth(0)
        .ok_or_else(|| "Could not find magnet link".to_owned())?;
    // table data is |Type|Name|Seeders|Leechers|
    let tds = row.find(Name("td"));
    let mut tds = tds.skip(2);
    let seeders = tds.next().and_then(|v| v.text().parse::<u32>().ok());
    let leechers = tds.next().and_then(|v| v.text().parse::<u32>().ok());

    let magnet_link = link
        .attr("href")
        .ok_or_else(|| "Could not find href element".to_owned())?;

    Ok(Torrent {
        name,
        magnet_link: magnet_link.to_owned(),
        seeders,
        leechers,
    })
}

fn parse_piratebay(document: &Document) -> Vec<Torrent> {
    let search_result = document.find(Attr("id", "searchResult")).into_selection();

    let mut result = vec![];
    // iterate table rows but skip header
    for table_row in search_result.find(Name("tr")).iter().skip(1) {
        match parse_piratebay_entry(&table_row) {
            Ok(torrent) => result.push(torrent),
            Err(e) => error!("{}", e),
        }
    }
    result
}

#[cfg(test)]
mod test {
    use select::document::Document;
    static TEST_DATA: &str = include_str!("test_data/piratebay.html");

    #[test]
    fn test_parse_piratebay() {
        let document = Document::from(TEST_DATA);
        let torrents = super::parse_piratebay(&document);
        assert_eq!(torrents.len(), 16);
        for torrent in torrents.iter() {
            assert!(torrent.magnet_link.starts_with("magnet:?"));
            assert!(torrent.seeders.is_some());
            assert!(torrent.leechers.is_some());
        }
    }
}
