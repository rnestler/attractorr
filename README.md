[![Travis Build Status](https://travis-ci.org/rnestler/rust-torrent-search.svg?branch=master)](https://travis-ci.org/rnestler/rust-torrent-search)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/6xjt3r1l1hqbm116/branch/master?svg=true)](https://ci.appveyor.com/project/rnestler/rust-torrent-search)
# torrent-search

This is a tool, written in Rust, to search for torrents on the command line.

## Building

`rust-torrent-search` needs an external [openssl library](https://openssl.org),
make sure to [install it](https://github.com/sfackler/rust-openssl#building).

### Linux

Just install openssl with your package manager:

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

### Windows

To install openssl on Windows see [here](https://github.com/sfackler/rust-openssl#windows)

## Usage
There are two interfaces:
 * torrent-search-cli
 * torrent-search-gtk

When using cargo one has to use the `--bin` option to select the binary to run:

```
# search for "search term" in all supported sources.
$ cargo run --bin torrent-search-cli $searchterm
# start the GTK GUI
$ cargo run --bin torrent-search-gtk
```

