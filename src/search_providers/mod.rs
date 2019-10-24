use std::error::Error;
use torrent::Torrent;

pub mod kickass_search;
pub mod pirate_bay_search;

pub trait SearchProvider {
    fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error>>;
}
