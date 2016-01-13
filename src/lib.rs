extern crate hyper;
extern crate select;

pub mod torrent;

pub mod search_providers;
pub use search_providers::SearchProvider;
pub use search_providers::pirate_bay_search::PirateBaySearch;
pub use search_providers::kickass_search::KickassSearch;

pub fn get_search_providers() -> Vec<Box<SearchProvider>> {
    vec![
        Box::new(PirateBaySearch::new()),
        Box::new(KickassSearch::new()),
    ]
}

