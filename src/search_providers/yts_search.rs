use crate::torrent::Torrent;
use crate::SearchProvider;

use async_trait::async_trait;

use log::info;
use std::error::Error;
use yts_api::{ListMovies, MovieList};

pub struct YtsSearch {}

impl YtsSearch {
    pub fn new() -> YtsSearch {
        YtsSearch {}
    }
}

#[async_trait]
impl SearchProvider for YtsSearch {
    async fn search(&self, term: &str) -> Result<Vec<Torrent>, Box<dyn Error + Send + Sync>> {
        info!("Searching on YTS");
        let res = ListMovies::new().query_term(term).execute().await?;

        Ok(parse_yts(res))
    }

    fn get_name(&self) -> &'static str {
        "YTS"
    }
}

fn parse_yts(movie_list: MovieList) -> Vec<Torrent> {
    let mut results_output: Vec<Torrent> = Vec::new();

    info!("Count: {}", movie_list.movie_count);

    for movie in movie_list.movies {
        let name = movie.title;
        for torrent in movie.torrents {
            results_output.push(Torrent {
                name: name.clone(),
                magnet_link: format!("magnet:?xt=urn:btih:{}", torrent.hash),
                seeders: Some(torrent.seeds),
                leechers: Some(torrent.peers),
                ..Default::default()
            });
        }
    }

    results_output
}
