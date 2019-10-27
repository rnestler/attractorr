extern crate hyper;
extern crate hyper_native_tls;
extern crate select;

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

use std::error::Error;
use std::io::Read;

use log::{error, info};

use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;
use hyper::header::{Connection, UserAgent};
use hyper::Client;

use search_providers::SearchProvider;
use torrent::Torrent;

pub struct KickassSearch {
    connection: hyper::Client,
}

impl KickassSearch {
    pub fn new() -> KickassSearch {
        let tls = NativeTlsClient::new().unwrap();
        KickassSearch {
            connection: Client::with_connector(HttpsConnector::new(tls)),
        }
    }
}

impl SearchProvider for KickassSearch {
    fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error>> {
        info!("Searching on Kickass");
        let mut res = self
            .connection
            .get(&format!("https://katcr.co/katsearch/page/1/{}", term))
            .header(Connection::close())
            .header(UserAgent(
                "Mozilla/5.0 (X11; Linux x86_64; rv:70.0) Gecko/20100101 Firefox/70.0".into(),
            ))
            .send()?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let document = Document::from(&*body);
        Ok(parse_kickass(&document))
    }

    fn get_name(&self) -> &'static str {
        "KAT"
    }
}

fn parse_kickass_entry(row: &Node) -> Result<Torrent, String> {
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
    static TEST_DATA: &'static str = include_str!("test_data/kickass.html");

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
