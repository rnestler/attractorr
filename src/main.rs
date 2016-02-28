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
    let providers: Vec<Box<SearchProvider>> = vec![
        Box::new(PirateBaySearch::new()),
        Box::new(KickassSearch::new()),
    ];

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
        if let Some(seeders) = torrent.seeders {
            if let Some(leechers) = torrent.leechers {
                print!("S:{}/L:{} - ", seeders, leechers);
            } else {
                print!("S:{} - ", seeders);
            }
        }
        println!("{}", torrent.name);
        println!("{}", torrent.magnet_link);
        println!("");
    }
}
