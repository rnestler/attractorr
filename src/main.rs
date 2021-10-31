mod search_providers;
use search_providers::l337x_search::L337xSearch;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::yts_search::YtsSearch;
use search_providers::{search_providers_from_ids, SearchProvider, SearchProviderId};

mod torrent;
use torrent::Torrent;

use atty::Stream;
use futures_util::future::join_all;
use log::{debug, error};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum SortMethods {
        Seeders,
        Leechers,
    }
}

arg_enum! {
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub enum ColorOptions {
        Always,
        Auto,
        Never,
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    /// Sort results by the number of seeders or leechers.
    #[structopt(long, possible_values = &SortMethods::variants(), case_insensitive = true)]
    sort: Option<SortMethods>,

    /// Use the given search providers
    #[structopt(long, multiple = false, use_delimiter = true, possible_values = &SearchProviderId::variants(), case_insensitive = true)]
    search_providers: Vec<SearchProviderId>,

    /// Control whether to use color
    #[structopt(long, value_name = "WHEN", possible_values = &ColorOptions::variants(), case_insensitive = true, default_value="auto")]
    color: ColorOptions,

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

    let colorful = match args.color {
        ColorOptions::Never => false,
        ColorOptions::Always => true,
        // Enable colors if:
        // * NO_COLOR env var isn't set: https://no-color.org/
        // * The output stream is stdout (not being piped)
        ColorOptions::Auto => std::env::var_os("NO_COLOR").is_none() && atty::is(Stream::Stdout),
    };

    // create all search providers
    let providers: Vec<Box<dyn SearchProvider>> = if args.search_providers.is_empty() {
        vec![
            Box::new(PirateBaySearch::new()),
            Box::new(L337xSearch::new()),
            Box::new(YtsSearch::new()),
        ]
    } else {
        search_providers_from_ids(&args.search_providers)
    };

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

    if torrents.is_empty() {
        eprintln!("No results were found");
    }

    // print out all torrents
    for torrent in torrents.iter() {
        torrent.print(colorful);
    }

    Ok(())
}
