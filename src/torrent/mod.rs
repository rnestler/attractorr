extern crate ansi_term;

use self::ansi_term::Colour::{Red, Green};

#[derive(Debug)]
pub struct Torrent {
    pub name: String,
    pub magnet_link: String,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
}

impl Torrent {
    pub fn print(&self) {
        if let Some(seeders) = self.seeders {
            print!("{}", Green.paint(format!("S:{}", seeders)));
            if let Some(leechers) = self.leechers {
                print!("/{}", Red.paint(format!("L:{}", leechers)));
            }
            print!(" - ");
        }
        println!("{}", self.name);
        println!("{}", self.magnet_link);
        println!("");
    }
}
