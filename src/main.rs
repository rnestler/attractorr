mod search_providers;
use search_providers::l337x_search::L337xSearch;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::yts_search::YtsSearch;
use search_providers::{search_providers_from_ids, SearchProvider, SearchProviderId};

mod torrent;
use torrent::Torrent;

use atty::Stream;
use clap::Parser;
use futures_util::future::join_all;
use log::{debug, error};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use termcolor::ColorChoice;

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum SortMethods {
    Seeders,
    Leechers,
}

#[derive(clap::ValueEnum, Debug, Eq, PartialEq, Copy, Clone)]
pub enum ColorOptions {
    Always,
    Auto,
    Never,
}

#[derive(Debug, Parser)]
struct Args {
    /// Sort results by the number of seeders or leechers.
    #[clap(long, value_enum, ignore_case = true)]
    sort: Option<SortMethods>,

    /// Use the given search providers
    #[clap(long, use_value_delimiter = true, value_enum, ignore_case = true)]
    search_providers: Vec<SearchProviderId>,

    /// Control whether to use color
    #[clap(
        long,
        value_name = "WHEN",
        value_enum,
        ignore_case = true,
        default_value = "auto"
    )]
    color: ColorOptions,

    #[clap(name = "SEARCHTERM", required = true, min_values = 1)]
    searchterm: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    // parse arguments
    let args = Args::parse();

    debug!("Searchterm: {:?}", args.searchterm);

    let keyword = utf8_percent_encode(&args.searchterm.join(" "), NON_ALPHANUMERIC).to_string();
    let sort_method = args.sort;

    let color_choice = match args.color {
        ColorOptions::Never => ColorChoice::Never,
        ColorOptions::Always => ColorChoice::Always,
        // If stdout is a tty (not being piped) let termcolor decide if we should use colors. See
        // https://docs.rs/termcolor/1.1.3/termcolor/index.html#detecting-presence-of-a-terminal
        ColorOptions::Auto => {
            if atty::is(Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
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
        torrent.print(color_choice);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
