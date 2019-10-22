extern crate hyper;
extern crate select;
extern crate flate2;

use self::flate2::read::GzDecoder;

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

use std::error::Error;
use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

use torrent::Torrent;
use search_providers::SearchProvider;


pub struct KickassSearch {
    connection: hyper::Client
}

impl KickassSearch {
    pub fn new() -> KickassSearch {
        KickassSearch{connection: Client::new()}
    }
}

impl SearchProvider for KickassSearch {
    fn search(&self, term: &str) -> Result<Vec<Torrent>,Box<Error>> {
        let res = try!(self.connection.get(&format!("https://kat.cr/usearch/{}", term))
            .header(Connection::close())
            .send());

        let mut body = String::new();
        let mut d = GzDecoder::new(res).unwrap();
        try!(d.read_to_string(&mut body));

        let document = Document::from(&*body);
        Ok(parse_kickass(&document))
    }
}

fn parse_kickass_entry(row: &Node) -> Result<Torrent, String> {
    let name = try!(row.find(Class("cellMainLink")).first()
                    .ok_or("Could not find 'cellMainLink'".to_owned())
                    .and_then(|n| Ok(n.text()))
                   );

    let link = try!(row.find(Attr("title", "Torrent magnet link")).first()
                    .ok_or("Could not find magnet link".to_owned())
                    );

    let magnet_link = try!(link.attr("href").ok_or("Could not find href element".to_owned()));

    // table data is |Name|Size|Files|Age|Seeders|Leechers|
    let tds = row.find(Name("td"));
    let mut tds = tds.iter().skip(4);
    let seeders = tds.next()
        .and_then(|v| v.text().parse::<u32>().ok());
    let leechers = tds.next()
        .and_then(|v| v.text().parse::<u32>().ok());

    Ok(Torrent{name: name, magnet_link: magnet_link.to_owned(), seeders: seeders, leechers: leechers})
}

fn parse_kickass(document: &Document) -> Vec<Torrent> {
    let search_result = document.find(Attr("class", "data"));

    let mut result = vec![];
    // iterate table rows but skip header
    for table_row in search_result.find(Name("tr")).iter().skip(1) {
        match parse_kickass_entry(&table_row) {
            Ok(torrent) => result.push(torrent),
            Err(e) => println!("{}", e),
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
        assert_eq!(torrents.len(), 25);
        for torrent in torrents.iter() {
            assert!(torrent.magnet_link.starts_with("magnet:?"));
            assert!(torrent.seeders.is_some());
            assert!(torrent.leechers.is_some());
        }
    }
}

