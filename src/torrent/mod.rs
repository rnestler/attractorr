use ansi_term::{Color, Style};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Torrent {
    pub name: String,
    pub magnet_link: String,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
}

impl Torrent {
    pub fn print(&self, colorful: bool) {
        let (seeders_style, leechers_style) = if colorful {
            (Style::new().fg(Color::Green), Style::new().fg(Color::Red))
        } else {
            (Style::default(), Style::default())
        };

        if let (Some(seeders), Some(leechers)) = (self.seeders, self.leechers) {
            print!(
                "{}/{} - ",
                seeders_style.paint(format!("S:{}", seeders)),
                leechers_style.paint(format!("L:{}", leechers))
            );
        }
        println!("{}", self.name);
        println!("{}", self.magnet_link);
        println!();
    }

    pub fn compare_seeders(&self, other: &Torrent) -> Ordering {
        self.seeders.cmp(&other.seeders)
    }

    pub fn compare_leechers(&self, other: &Torrent) -> Ordering {
        self.leechers.cmp(&other.leechers)
    }
}
