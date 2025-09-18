use crate::SearchProvider;
use crate::torrent::Torrent;

use async_trait::async_trait;
use log::{info, warn};
use std::error::Error;
use torrent_search::{TorrentSearchResult, search_l337x};

//The struct field is needed for compatibility
pub struct L337xSearch {}

impl L337xSearch {
    pub fn new() -> L337xSearch {
        L337xSearch {}
    }
}

#[async_trait]
impl SearchProvider for L337xSearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on L337X");
        let res = search_l337x(term.to_string()).await?;

        Ok(parse_l337x(res))
    }

    fn get_name(&self) -> &'static str {
        "L337X"
    }
}

fn parse_l337x(mut results: Vec<TorrentSearchResult>) -> Vec<Torrent> {
    let mut results_output: Vec<Torrent> = Vec::new();

    for result in results.drain(..) {
        results_output.push(Torrent {
            name: result.name.clone(),
            magnet_link: result.magnet.as_ref().unwrap().to_string(),
            seeders: result
                .seeders
                .map_err(|e| warn!("Getting seeders failed: {}", e))
                .ok(),
            leechers: result
                .leeches
                .map_err(|e| warn!("Getting leechers failed: {}", e))
                .ok(),
            size_bytes: None,
            source: "L337X",
        });
    }

    results_output
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_parse_l337x() {
        let torrents = vec![TorrentSearchResult {
            name: "test".into(),
            seeders: Ok(1),
            leeches: Ok(2),
            magnet: Ok("magnet:?".into()),
        }];
        let torrents = parse_l337x(torrents);

        assert_eq!(1, torrents.len());
        assert_eq!(torrents[0].name, "test");
        assert_eq!(torrents[0].magnet_link, "magnet:?");
        assert_eq!(torrents[0].seeders, Some(1));
        assert_eq!(torrents[0].leechers, Some(2));
    }
}
