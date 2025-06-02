use bytesize::ByteSize;
use std::cmp::Ordering;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Default)]
pub struct Torrent {
    pub name: String,
    pub magnet_link: String,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
    pub size_bytes: Option<u64>,
    pub source: &'static str,
}

impl Torrent {
    fn print_with_color(text: &str, color_choice: ColorChoice, color: Color) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(color_choice);
        stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(stdout, "{}", text)?;
        stdout.reset()
    }
    pub fn print(&self, color_choice: ColorChoice) {
        let seeders = self
            .seeders
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| "n/a".to_owned());
        let leechers = self
            .leechers
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| "n/a".to_owned());

        Self::print_with_color(&seeders, color_choice, Color::Green).ok();
        print!("/");
        Self::print_with_color(&leechers, color_choice, Color::Red).ok();
        print!(" - ");
        print!("{}", self.name);
        print!(" ({}", self.source);
        if let Some(size_bytes) = self.size_bytes {
            print!(", {}", ByteSize(size_bytes));
        }
        println!(")");
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
