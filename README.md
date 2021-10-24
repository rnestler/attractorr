[![CircleCI](https://circleci.com/gh/rnestler/rust-torrent-search.svg?style=svg)](https://circleci.com/gh/rnestler/rust-torrent-search)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/6xjt3r1l1hqbm116/branch/master?svg=true)](https://ci.appveyor.com/project/rnestler/rust-torrent-search)
# torrent-search

This is a tool, written in Rust, to search for torrents on the command line.

## Building

On Linux, make sure to have openssl installed:

```Bash
# ArchLinux
$ sudo pacman -S openssl
# Ubuntu
$ sudo apt-get install libssl-dev
```

Then just use [cargo](http://doc.crates.io/) to build

```
$ cargo build
```

## Install

```
cargo install --git https://github.com/rnestler/rust-torrent-search
```

## Usage

Just execute

```
$ cargo run "search term"
```

To search for "search term" in all supported sources.

## Alternatives

Here is a list of projects which do similar things:

 * https://crates.io/crates/tpb
 * https://crates.io/crates/torrentfind/
 * https://crates.io/crates/nyaadesu
 * https://crates.io/crates/ktorrent

## License

Licensed under either of

 * Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
