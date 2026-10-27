# Wordfreq

A simple CLI tool that shows the top N most frequent words in one or more files.

![Using the wordfreq CLI](docs/images/cli-usage.svg)

## Supported files

- .txt
- .md
- .pdf
- .docx
- text piped in on stdin (use `-` as the file name)

File extensions are not case sensitive, so `NOTES.TXT` works too.

## Build

```
cargo build --release
```

## Usage

```
wordfreq [OPTIONS] <FILES>...
```

`<FILES>` can be any mix of files, folders and `-` for stdin. All of them are counted together.

One file:

```
$ wordfreq files/cp.txt -n 3
Path: files/cp.txt
 1. n               8
 2. a               7
 3. standard        6
```

A folder, shown as a table:

```
$ wordfreq files -n 3 -f table
warning: File files/empty_file.txt is completely empty (skipped)
warning: File files/empty_pdf.pdf is completely empty (skipped)
Files: 5
┌───┬──────┬───────┬──────────────────────┐
│ # │ word │ count │                      │
├───┼──────┼───────┼──────────────────────┤
│ 1 │ the  │  6070 │ ████████████████████ │
│ 2 │ a    │  2535 │ ████████             │
│ 3 │ of   │  2220 │ ███████              │
└───┴──────┴───────┴──────────────────────┘
```

Two files as CSV:

```
$ wordfreq files/cp.txt files/dsa_patterns.docx -n 3 -f csv
rank,word,count
1,the,63
2,a,56
3,to,36
```

From stdin:

```
$ echo "to be or not to be" | wordfreq - -n 2
Path: -
 1. be              2
 2. to              2
```

## Options

| Option                  | What it does                                                                   |
| ----------------------- | ------------------------------------------------------------------------------ |
| `-r, --recursive`       | When a folder is given, also read the files in its subfolders.                 |
| `-f, --format <FORMAT>` | `plain` (default), `table` or `csv`.                                           |
| `-n, --top <N>`         | How many words to show. Default is 10.                                         |
| `-i, --case-sensitive`  | Count `The` and `the` as different words. By default all words are lowercased. |
| `--ignore-stop-words`   | Skip common words: the, for, on, in, and, a, is, to.                           |
| `-x, --exclude <FILE>`  | Skip the words listed in a file, one word per line.                            |
| `--min-len <N>`         | Only count words with at least N letters. Default is 1.                        |
| `--max-len <N>`         | Only count words with at most N letters.                                       |
| `-v, --verbose`         | Print files read, total words, unique words, diversity ratio and time.         |
| `-h, --help`            | Show help.                                                                     |
| `-V, --version`         | Show version.                                                                  |

`--min-len` and `--max-len` count letters, not bytes, so `café` has 4 letters.

`--ignore-stop-words` and `--exclude` can be used together.

## Files and folders

- A file given by name is always tried. If its type is not supported, that is reported (see "Output and errors").
- A folder adds the supported files directly inside it. Other files, like `.csv`, are skipped.
- With `-r`, subfolders are read too.
- Files are read in alphabetical order, so the output is the same on every run.

## How words are counted

- Text is split on spaces and new lines.
- Everything except letters, numbers and apostrophes is removed, so `hello,` becomes `hello` and `don't` stays `don't`.
- Words are lowercased unless `--case-sensitive` is used.
- Counts from all files are added together.
- Words with the same count are sorted alphabetically.

## Output and errors

Only the results go to stdout. Warnings, errors and the `-v` stats go to stderr. So you can pipe or save the results without anything else mixed in:

```
wordfreq docs/ -r -f csv > top.csv 2> errors.log
```

With `-f csv` there is no `Path:` or `Files:` line, only the CSV rows.

The tool stops with an error (exit code 1) when:

- `--min-len` is bigger than `--max-len`
- a folder cannot be read
- the `-x` file cannot be read
- none of the given files could be read

When you give one file, any problem with it is an error. When you give several files or a folder, a problem with one file is a warning and that file is skipped. These are the problems a file can have:

- it does not exist or cannot be read (the reason is shown, for example "Permission denied")
- it is not valid UTF-8 text
- it is a broken PDF or DOCX
- it is empty (0 bytes, or only spaces and new lines)
- its type is not supported

A wrong flag is reported by clap with exit code 2.

## How it works

Each module has one job. `main.rs` only connects them: it reads the options, collects the files, asks the parser for each file's text, runs the counting and filters from `core.rs`, and prints the result through `output.rs`.

![wordfreq architecture](docs/images/architecture.svg)

| File | Job |
| --- | --- |
| `src/main.rs` | Runs each step in order. |
| `src/cli.rs` | The command line options (clap). |
| `src/input.rs` | `collect_files`: turns files, folders and `-r` into a list of files. |
| `src/parser/mod.rs` | `extract_text`: checks a file and picks a parser by extension, or reads stdin. |
| `src/parser/txt.rs`, `pdf.rs`, `docx.rs` | Turn one file type into plain text. |
| `src/core.rs` | Counting, merging, filters, top N and stats. No file access. |
| `src/output.rs` | Writes the results as plain, table or csv. |
| `src/error.rs` | The `CliError` type used for every error. |

This is what happens on every run, and where each error comes from:

![wordfreq control flow](docs/images/control-flow.svg)
