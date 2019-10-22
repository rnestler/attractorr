extern crate docopt;
extern crate rustc_serialize;

mod torrent;
use torrent::Torrent;

mod search_providers;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::SearchProvider;

static USAGE: &'static str = "
Usage:
  torrent-search [options] <searchterm>...
  torrent-search (-h | --help)

Options:
  -h --help             Show this screen.
  --sort=SORTMETHOD     Sort results by the number of: seeders or leechers.
";

#[derive(Debug, RustcDecodable)]
enum SortMethods {
    Seeders,
    Leechers,
}

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_searchterm: Vec<String>,
    flag_sort: Option<SortMethods>,
}

fn main() {
    // parse arguments
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let keyword = &args.arg_searchterm.join(" ");
    let sort_method = args.flag_sort;

    // create all search providers
    let providers: Vec<Box<dyn SearchProvider>> = vec![Box::new(PirateBaySearch::new())];

    // search for torrents
    let mut torrents = vec![];
    for provider in providers.iter() {
        match provider.search(&keyword) {
            Ok(results) => torrents.extend(results),
            Err(err) => println!("Error: {}", err),
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
}
