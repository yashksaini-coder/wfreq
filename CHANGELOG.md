# Changelog

All notable changes to this project are listed here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.0] - 2026-10-27

### Changed

- **Breaking:** the command is now `wfreq` instead of `wordfreq`, matching the crate name.
  Install with `cargo install wfreq` and run `wfreq <FILES>...`.
- **Breaking:** the library is now `wfreq`, so imports change from `use wordfreq::...` to `use wfreq::...`.
- README header is a screenshot of real output; diagrams use the new name.

## [0.1.0] - 2026-10-27

First release, published on crates.io as `wfreq`. In this version the command is called `wordfreq`.

### Added

- Count the top N words in `.txt`, `.md`, `.pdf` and `.docx` files.
- Read several files at once, whole folders, subfolders with `-r`, and stdin with `-`.
- `--format plain|table|csv` output. The table draws a bar for each count.
- Filters: `--ignore-stop-words`, `-x/--exclude <FILE>`, `--min-len`, `--max-len`, `-i/--case-sensitive`.
- `-v/--verbose` stats on stderr: files read, total words, unique words, diversity ratio, time.
- Clear errors with the file path and the reason. With several files, a bad file is a warning and is skipped.
- Library API (`wfreq::core`, `parser`, `input`, `output`) with rustdoc on every public item.
- Unit, integration, property (proptest) and doc tests; criterion benchmarks.
- CI: fmt, clippy with warnings denied, tests, rustdoc and minimum Rust version 1.88.

[Unreleased]: https://github.com/yashksaini-coder/wordfreq/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/yashksaini-coder/wordfreq/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yashksaini-coder/wordfreq/releases/tag/v0.1.0
