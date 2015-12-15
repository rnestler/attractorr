extern crate hyper;
extern crate select;

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

#[derive(Debug)]
struct Torrent {
    pub name: String,
    pub magnet_link: String,
    pub seeders: Option<u32>,
    pub leachers: Option<u32>,
}

fn parse_piratebay_entry(row: &Node) -> Result<Torrent, String> {
    let name = try!(row.find(Class("detLink")).first()
                    .ok_or("Could not find 'detLink'".to_owned())
                    .and_then(|n| Ok(n.text()))
                   );

    let link = try!(row.find(Attr("title", "Download this torrent using magnet")).first()
                    .ok_or("Could not find magnet link".to_owned())
                    );

    let magnet_link = try!(link.attr("href").ok_or("Could not find href element".to_owned()));

    Ok(Torrent{name: name, magnet_link: magnet_link.to_owned(), seeders: None, leachers: None})
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

fn main() {
    let client = Client::new();

    let mut res = client.get("https://thepiratebay.mn/search/debian/0/99/0")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let document = Document::from_str(&body);
    let entries = parse_piratebay(&document);

    for entry in entries.iter() {
        println!("{:?}", entry);
    }
}

