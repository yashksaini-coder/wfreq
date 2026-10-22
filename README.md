# Wordfreq

A simple CLI tool that shows the top N most frequent words in a file.

## Supported files

- .txt
- .md
- .pdf (planned)
- .docx (planned)

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
| `-v, --verbose`        | Also print the number of unique words and how long it took.                    |
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
- the file is empty
- the file type is not supported
- `--min-len` is bigger than `--max-len`
