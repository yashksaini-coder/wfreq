# Wordfreq

A simple CLI tool that shows the top N most frequent words in a file.

![Using the wordfreq CLI](docs/images/cli-usage.svg)

## Supported files

- .txt
- .md
- .pdf
- .docx

File extensions are not case sensitive, so `NOTES.TXT` works too.

## Build

```
cargo build --release
```

## Usage

```
wordfreq [OPTIONS] <FILE>
```

Example:

```
$ wordfreq files/cp.txt -n 3
Path: files/cp.txt
 1. n               8
 2. a               7
 3. standard        6
```

## Options

| Option                 | What it does                                                                   |
| ---------------------- | ------------------------------------------------------------------------------ |
| `-n, --top <N>`        | How many words to show. Default is 10.                                         |
| `-i, --case-sensitive` | Count `The` and `the` as different words. By default all words are lowercased. |
| `--ignore-stop-words`  | Skip common words: the, for, on, in, and, a, is, to.                           |
| `-x, --exclude <FILE>` | Skip the words listed in a file, one word per line.                            |
| `--min-len <N>`        | Only count words with at least N letters. Default is 1.                        |
| `--max-len <N>`        | Only count words with at most N letters.                                       |
| `-v, --verbose`        | Also print total words, unique words, diversity ratio and how long it took.    |
| `-h, --help`           | Show help.                                                                     |
| `-V, --version`        | Show version.                                                                  |

`--min-len` and `--max-len` count letters, not bytes, so `café` has 4 letters.

`--ignore-stop-words` and `--exclude` can be used together.

The `-r, --recursive` flag is shown in `--help` but does not do anything yet.

## How words are counted

- Text is split on spaces and new lines.
- Everything except letters, numbers and apostrophes is removed, so `hello,` becomes `hello` and `don't` stays `don't`.
- Words are lowercased unless `--case-sensitive` is used.
- Words with the same count are sorted alphabetically, so the output is the same on every run.

## Errors

Errors are printed to stderr and the program exits with code 1. Results go to stdout. This means you can pipe the results into other tools without the error messages getting mixed in.

```
wordfreq notes.txt > top.txt 2> errors.log
```

The tool gives an error when:

- the file does not exist or cannot be read (the reason is shown, for example "Permission denied")
- the file is not valid UTF-8 text
- a PDF or DOCX file is broken and cannot be parsed
- the file is empty (0 bytes, or only spaces and new lines)
- the file type is not supported
- `--min-len` is bigger than `--max-len`

## How it works

Each module has one job. `main.rs` only connects them: it reads the options, asks the parser for the text, runs the counting and filters from `core.rs`, and prints the result.

![wordfreq architecture](docs/images/architecture.svg)

| File | Job |
| --- | --- |
| `src/main.rs` | Runs each step in order. Prints results to stdout and errors to stderr. |
| `src/cli.rs` | The command line options (clap). |
| `src/parser/mod.rs` | `extract_text`: checks the file and picks a parser by extension. |
| `src/parser/txt.rs`, `pdf.rs`, `docx.rs` | Turn one file type into plain text. |
| `src/core.rs` | Counting, filters, top N and stats. No file access. |
| `src/error.rs` | The `CliError` type used for every error. |

This is what happens on every run, and where each error comes from:

![wordfreq control flow](docs/images/control-flow.svg)
