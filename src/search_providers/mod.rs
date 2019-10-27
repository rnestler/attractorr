use crate::torrent::Torrent;
use async_trait::async_trait;
use std::error::Error;

pub mod kickass_search;
pub mod pirate_bay_search;

#[async_trait]
pub trait SearchProvider {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>>;
    fn get_name(&self) -> &'static str;
}
