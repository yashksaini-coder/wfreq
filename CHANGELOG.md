# Changelog

All notable changes to this project are listed here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.0] - 2026-10-27

First release, published on crates.io as `wfreq`.

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

[Unreleased]: https://github.com/yashksaini-coder/wordfreq/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yashksaini-coder/wordfreq/releases/tag/v0.1.0
