use crate::torrent::Torrent;
use crate::SearchProvider;

use async_trait::async_trait;
use log::info;
use std::convert::TryInto;
use std::error::Error;
use torrent_search::*;

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
        let res = search_l337x(term.to_string()).await.unwrap_or_default();
        //info!("Status: {}", res.status());

        Ok(parse_l337x(res))
    }

    fn get_name(&self) -> &'static str {
        "L337X"
    }
}

fn parse_l337x(results: Vec<TorrentSearchResult>) -> Vec<Torrent> {
    let mut results_output: Vec<Torrent> = Vec::new();

    for result in results.iter() {
        results_output.push(Torrent {
            name: result.name.clone(),
            magnet_link: result.magnet.as_ref().unwrap().to_string(),
            seeders: match result.seeders {
                Ok(s) => Some(s.try_into().unwrap()),
                _ => None,
            },
            leechers: match result.leeches {
                Ok(l) => Some(l.try_into().unwrap()),
                _ => None,
            },
        });
    }

    results_output
}

#[cfg(test)]
mod test {
    use torrent_search::search_l337x;

    #[tokio::test]
    async fn test_parse_l337x() {
        let torrents = search_l337x("Debian".to_string()).await.unwrap_or_default();

        for torrent in torrents.iter() {
            assert!(&torrent.magnet.as_ref().unwrap().starts_with("magnet:?"));
            assert!(torrent.seeders.is_ok());
            assert!(torrent.leeches.is_ok());
        }
    }
}
