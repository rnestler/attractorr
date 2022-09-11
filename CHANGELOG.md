# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2022-09-11

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


[Unreleased]: https://github.com/rnestler/attractorr/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/rnestler/attractorr/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/rnestler/attractorr/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rnestler/attractorr/releases/tag/v0.1.0
