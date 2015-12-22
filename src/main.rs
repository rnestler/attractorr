extern crate hyper;
extern crate select;
extern crate docopt;

mod torrent;

mod search_providers;
use search_providers::SearchProvider;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::kickass_search::KickassSearch;


static USAGE: &'static str = "
Usage: torrent-search <searchterm>
";

fn main() {
    let args = docopt::Docopt::new(USAGE).and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    // create all search providers
    let mut providers = vec![];
    providers.push(PirateBaySearch::new());
    //providers.push(KickassSearch::new());

    // search for torrents
    let keyword = args.get_str("<searchterm>");
    let mut torrents = vec![];
    for provider in providers.iter() {
        torrents.push(provider.search(keyword));
    }

    // print out all torrents
    for torrent in torrents.iter() {
        println!("{:#?}", torrent);
    }
}
