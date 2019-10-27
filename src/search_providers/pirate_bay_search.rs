use hyper;
use hyper_native_tls;
use select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Attr, Class, Name};

use std::error::Error;
use std::io::Read;

use log::{error, info};

use self::hyper::header::Connection;
use self::hyper::net::HttpsConnector;
use self::hyper::Client;
use self::hyper_native_tls::NativeTlsClient;

use crate::search_providers::SearchProvider;
use crate::torrent::Torrent;

pub struct PirateBaySearch {
    connection: Client,
}

impl PirateBaySearch {
    pub fn new() -> PirateBaySearch {
        let tls = NativeTlsClient::new().unwrap();
        PirateBaySearch {
            connection: Client::with_connector(HttpsConnector::new(tls)),
        }
    }
}

impl SearchProvider for PirateBaySearch {
    fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error>> {
        info!("Searching on PirateBay");
        let mut res = self
            .connection
            .get(&format!("https://thepiratebay.org/search/{}/0/99/0", term))
            .header(Connection::close())
            .send()?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;

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
    use super::select::document::Document;
    static TEST_DATA: &'static str = include_str!("test_data/piratebay.html");

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
