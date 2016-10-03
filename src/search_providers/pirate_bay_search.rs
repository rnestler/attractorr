extern crate hyper;
extern crate select;

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

use std::error::Error;
use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

use torrent::Torrent;
use search_providers::SearchProvider;


pub struct PirateBaySearch {
    connection: hyper::Client
}

impl PirateBaySearch {
    pub fn new() -> PirateBaySearch {
        PirateBaySearch{connection: Client::new()}
    }
}

impl SearchProvider for PirateBaySearch {
    fn search(&self, term: &str) -> Result<Vec<Torrent>,Box<Error>> {
        let mut res = try!(self.connection.get(&format!("https://thepiratebay.se/search/{}/0/99/0", term))
            .header(Connection::close())
            .send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let document = Document::from(&*body);
        Ok(parse_piratebay(&document))
    }
}

fn parse_piratebay_entry(row: &Node) -> Result<Torrent, String> {
    let name = try!(row.find(Class("detLink")).first()
                    .ok_or("Could not find 'detLink'".to_owned())
                    .and_then(|n| Ok(n.text()))
                   );

    let link = try!(row.find(Attr("title", "Download this torrent using magnet")).first()
                    .ok_or("Could not find magnet link".to_owned())
                    );
    // table data is |Type|Name|Seeders|Leechers|
    let tds = row.find(Name("td"));
    let mut tds = tds.iter().skip(2);
    let seeders = tds.next()
        .and_then(|v| v.text().parse::<u32>().ok());
    let leechers = tds.next()
        .and_then(|v| v.text().parse::<u32>().ok());

    let magnet_link = try!(link.attr("href").ok_or("Could not find href element".to_owned()));

    Ok(Torrent{name: name, magnet_link: magnet_link.to_owned(), seeders: seeders, leechers: leechers})
}

fn parse_piratebay(document: &Document) -> Vec<Torrent> {
    let search_result = document.find(Attr("id", "searchResult"));

    let mut result = vec![];
    // iterate table rows but skip header
    for table_row in search_result.find(Name("tr")).iter().skip(1) {
        match parse_piratebay_entry(&table_row) {
            Ok(torrent) => result.push(torrent),
            Err(e) => println!("{}", e),
        }
    }
    result
}

#[cfg(test)]
mod test {
    use select::document::Document;
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

