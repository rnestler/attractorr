use crate::torrent::Torrent;
use async_trait::async_trait;
use std::error::Error;

pub mod l337x_search;
pub mod pirate_bay_search;
pub mod yts_search;

use l337x_search::L337xSearch;
use pirate_bay_search::PirateBaySearch;
use yts_search::YtsSearch;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
pub enum SearchProviderId {
    L337x,
    PirateBay,
    Yts,
}

#[async_trait]
pub trait SearchProvider {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>>;
    fn get_name(&self) -> &'static str;
}

pub fn search_provider_from_id(id: &SearchProviderId) -> Box<dyn SearchProvider> {
    match id {
        SearchProviderId::L337x => Box::new(L337xSearch::new()),
        SearchProviderId::PirateBay => Box::new(PirateBaySearch::new()),
        SearchProviderId::Yts => Box::new(YtsSearch::new()),
    }
}

pub fn search_providers_from_ids(ids: &[SearchProviderId]) -> Vec<Box<dyn SearchProvider>> {
    ids.iter().map(search_provider_from_id).collect()
}
