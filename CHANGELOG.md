# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.2]

### Changed

 * Bump MSRV to 1.74
 * Update dependencies

### Added

 * Add `--version` flag and about text to CLI

## [0.6.1] - 2024-01-03

### Fixed

 * Fix torrent-csv search

### Changed

 * Update dependencies

## [0.6.0] - 2023-10-19

### Changed

 * Print size of torrents when supported
 * Bump MSRV to 1.70
 * Update dependencies

## [0.5.3] - 2023-07-11

### Changed

 * Update dependencies

## [0.5.2] - 2023-06-03

### Changed

 * Update dependencies

## [0.5.1] - 2023-04-04

### Changed

 * Update dependencies

## [0.5.0] - 2023-02-15

### Added

 * Added torrent-csv as a new search provider.

### Changed

 * Bump MSRV to 1.64
 * Update dependencies

## [0.4.2] - 2022-12-01

### Changed

 * Update to clap 4.0 for command line parsing. While this shouldn't change
   behaviour, it changed the format of the `--help` output.
 * Bump MSRV to 1.60
 * Update dependencies

## [0.4.1] - 2022-10-03

### Changed

 * Update dependencies
 * Switch CI to GitHub actions
 * Mention how to install from AUR in README

## [0.4.0] - 2022-09-24

### Changed

 * Use clap directly for command line parsing, which leads to a different
   `--help` output for the `--sort` and `--search-providers` options.
 * Bump MSRV to 1.57
 * Switch from `ansi_term` to `termcolor`

### Breaking

 * `piratebay` changed to `pirate-bay` when specifing it as a search provider
   with `--search-providers` 

## [0.3.0] - 2022-09-12

### Changed

 * Rename project to Attractorr
 * Update dependencies
 * Bump MSRV to 1.56
 * Make `search_providers::pirate_bay_search::Entry` fields public

## [0.2.0] - 2021-10-31

### Added 

 * Allow to specify search providers on command line
 * Allow to disable colored output

### Changed

 * Build with msvc toolchain on Windows
 * Use structopt instead of docopt for command line parsing
 * Print a warning to stderr when no results were found

### Fixed

 * Do not output error message pseudo torrent from piratebay

## [0.1.0] - 2021-10-23

 * Initial release


[Unreleased]: https://github.com/rnestler/attractorr/compare/v0.6.2...HEAD
[0.6.2]: https://github.com/rnestler/attractorr/compare/v0.6.0...v0.6.2
[0.6.1]: https://github.com/rnestler/attractorr/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/rnestler/attractorr/compare/v0.5.3...v0.6.0
[0.5.3]: https://github.com/rnestler/attractorr/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/rnestler/attractorr/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/rnestler/attractorr/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/rnestler/attractorr/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/rnestler/attractorr/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/rnestler/attractorr/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/rnestler/attractorr/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/rnestler/attractorr/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/rnestler/attractorr/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rnestler/attractorr/releases/tag/v0.1.0
