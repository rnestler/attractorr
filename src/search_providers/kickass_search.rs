extern crate hyper;
extern crate select;
extern crate flate2;

use self::flate2::read::GzDecoder;

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

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
    fn search(&self, term: &str) -> Vec<Torrent> {
        let res = self.connection.get(&format!("https://kat.cr/usearch/{}", term))
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        let mut d = GzDecoder::new(res).unwrap();
        d.read_to_string(&mut body).unwrap();

        let document = Document::from_str(&body);
        parse_kickass(&document)
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

    Ok(Torrent{name: name, magnet_link: magnet_link.to_owned(), seeders: None, leachers: None})
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
        let document = Document::from_str(TEST_DATA);
        let torrents = super::parse_kickass(&document);
        assert_eq!(torrents.len(), 25);
    }
}

