use crate::torrent::Torrent;
use std::error::Error;

pub mod kickass_search;
pub mod pirate_bay_search;

pub trait SearchProvider {
    fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error>>;
    fn get_name(&self) -> &'static str;
}
