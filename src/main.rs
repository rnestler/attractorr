mod search_providers;
use search_providers::kickass_search::KickassSearch;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::SearchProvider;

mod torrent;
use torrent::Torrent;

use futures_util::future::join_all;
use log::error;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;

static USAGE: &str = "
Usage:
  torrent-search [options] <searchterm>...
  torrent-search (-h | --help)

Options:
  -h --help             Show this screen.
  --sort=SORTMETHOD     Sort results by the number of: seeders or leechers.
";

#[derive(Debug, Deserialize)]
enum SortMethods {
    Seeders,
    Leechers,
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_searchterm: Vec<String>,
    flag_sort: Option<SortMethods>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    // parse arguments
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let keyword = utf8_percent_encode(&args.arg_searchterm.join(" "), NON_ALPHANUMERIC).to_string();
    let sort_method = args.flag_sort;

    // create all search providers
    let providers: Vec<Box<dyn SearchProvider>> = vec![
        Box::new(PirateBaySearch::new()),
        Box::new(KickassSearch::new()),
    ];
    // search for torrents
    let providers = providers.iter().map(|provider| provider.search(&keyword));
    let results = join_all(providers).await;

    // collect torrents into one vec
    let mut torrents = vec![];
    for result in results {
        match result {
            Ok(t) => torrents.extend(t),
            Err(err) => error!("Error: {}", err),
        }
    }

    if let Some(sort_method) = sort_method {
        match sort_method {
            SortMethods::Seeders => torrents.sort_by(Torrent::compare_seeders),
            SortMethods::Leechers => torrents.sort_by(Torrent::compare_leechers),
        };
    }

    // print out all torrents
    for torrent in torrents.iter() {
        torrent.print();
    }
    Ok(())
}
