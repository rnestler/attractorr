use crate::torrent::Torrent;
use async_trait::async_trait;
use std::error::Error;

pub mod kickass_search;
pub mod l337x_search;
pub mod pirate_bay_search;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[async_trait]
pub trait SearchProvider {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>>;
    fn get_name(&self) -> &'static str;
}
