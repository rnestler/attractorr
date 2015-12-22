use torrent::Torrent;

pub mod pirate_bay_search;
pub mod kickass_search;

pub trait SearchProvider {
    fn search(&self, term: &str) -> Vec<Torrent>;
}
