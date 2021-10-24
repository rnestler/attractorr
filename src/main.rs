mod search_providers;
use search_providers::l337x_search::L337xSearch;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::yts_search::YtsSearch;
use search_providers::SearchProvider;

mod torrent;
use torrent::Torrent;

use futures_util::future::join_all;
use log::{debug, error};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum SortMethods {
    Seeders,
    Leechers,
}

impl std::str::FromStr for SortMethods {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seeders" => Ok(SortMethods::Seeders),
            "leechers" => Ok(SortMethods::Leechers),
            sort_method => Err(format!("Invalid sort method: {}", sort_method)),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    /// Sort results by the number of: seeders or leechers.
    #[structopt(long)]
    sort: Option<SortMethods>,

    #[structopt(name = "SEARCHTERM", required = true, min_values = 1)]
    searchterm: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    // parse arguments
    let args = Args::from_args();

    debug!("Searchterm: {:?}", args.searchterm);

    let keyword = utf8_percent_encode(&args.searchterm.join(" "), NON_ALPHANUMERIC).to_string();
    let sort_method = args.sort;

    // create all search providers
    let providers: Vec<Box<dyn SearchProvider>> = vec![
        Box::new(PirateBaySearch::new()),
        Box::new(L337xSearch::new()),
        Box::new(YtsSearch::new()),
    ];

    // search for torrents
    let results = providers.iter().map(|provider| provider.search(&keyword));
    let results = join_all(results).await;

    // collect torrents into one vec
    let mut torrents = vec![];
    for (result, provider) in results.into_iter().zip(providers) {
        match result {
            Ok(t) => torrents.extend(t),
            Err(err) => error!("{} error: {}", provider.get_name(), err),
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
