# Attractorr

[![GitHub Actions Build Status](https://github.com/rnestler/attractorr/actions/workflows/ci.yml/badge.svg)](https://github.com/rnestler/attractorr/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/attractorr.svg)](https://crates.io/crates/attractorr)
[![Crates.io Downloads](https://img.shields.io/crates/d/attractorr.svg)](https://crates.io/crates/attractorr)
[![AUR version](https://img.shields.io/aur/version/attractorr?label=AUR)](https://aur.archlinux.org/packages/attractorr)

Attractorr attracts magnets!

It is a tool, written in Rust, to search on various websites and APIs for
magnet links.

## Supported Websites

 * 1337x
 * TYS
 * piratebay
 * torrent-csv

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
cargo install attractorr
```

### Distro Packages

#### Arch Linux

On Arch Linux it is available from the
[AUR](https://aur.archlinux.org/packages/attractorr). You can use an [AUR
helper](https://wiki.archlinux.org/title/AUR_helpers) to install:

```
paru -S attractorr
```

## Usage

Just execute

```
$ cargo run "search term"
```

To search for "search term" in all supported sources.

With `--search-providers` one may limit the sources:
```
$ cargo run -- --sort seeders --search-providers yts,pirate-bay,torrent-csv test
```

## Alternatives

Here is a list of projects which do similar things:

 * https://crates.io/crates/tpb
 * https://crates.io/crates/torrentfind/
 * https://crates.io/crates/nyaadesu
 * https://crates.io/crates/ktorrent
 * https://crates.io/crates/magnetfinder
 * https://crates.io/crates/kickasstorrent

## License

Licensed under either of

 * Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
