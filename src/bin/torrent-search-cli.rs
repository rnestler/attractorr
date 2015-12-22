extern crate torrent_search;
extern crate docopt;

use torrent_search::SearchProvider;

static USAGE: &'static str = "
Usage: torrent-search-cli <searchterm>
";

fn main() {
    let args = docopt::Docopt::new(USAGE).and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    // create all search providers
    let providers = torrent_search::get_search_providers();

    // search for torrents
    let keyword = args.get_str("<searchterm>");
    let mut torrents = vec![];
    for provider in providers.iter() {
        match provider.search(keyword) {
            Ok(results) => torrents.extend(results),
            Err(err) => println!("Error: {}", err),
        }
    }

    // print out all torrents
    for torrent in torrents.iter() {
        println!("{:#?}", torrent);
    }
}
