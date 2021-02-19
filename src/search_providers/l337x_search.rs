#[cfg(test)]
mod test {
    use torrent_search::{search_l337x};

    #[test]
    fn test_parse_l337x() {
        let torrents = search_l337x("Debian".to_string()).unwrap_or(Vec::new());

        for torrent in torrents.iter() {
            assert!(&torrent.magnet.as_ref().unwrap().starts_with("magnet:?"));
            assert!(torrent.seeders.is_ok());
            assert!(torrent.leeches.is_ok());
        }
    }
}
