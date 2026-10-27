#!/usr/bin/env bash
# Compare wordfreq against the classic Unix word-count pipeline with hyperfine.
#
# Needs: pdftotext (poppler-utils) and hyperfine.
# Run from the repo root: ./benches/compare.sh
set -euo pipefail
export LC_ALL=C   # byte-wise sort/tr: the fastest setting for the Unix pipeline

corpus=target/bench/corpus.txt
mkdir -p target/bench

# ~17 MB of real English text: the Competitive Programmer's Handbook, repeated
if [ ! -f "$corpus" ]; then
    pdftotext files/Competitive-Programmer-Handbook.pdf target/bench/handbook.txt
    for _ in $(seq 40); do cat target/bench/handbook.txt; done > "$corpus"
fi
echo "corpus: $(du -h "$corpus" | cut -f1), $(wc -w < "$corpus") words"

cargo build --release --quiet

# Same job in both: lowercase, split on anything that is not a letter, digit or
# apostrophe, count, print the top 10.
hyperfine --shell=bash --warmup 2 --runs 10 \
    --command-name "wordfreq" \
    "./target/release/wordfreq $corpus -n 10" \
    --command-name "tr | sort | uniq -c | sort -rn" \
    "tr -cs \"[:alnum:]'\" '\n' < $corpus | tr '[:upper:]' '[:lower:]' | sort | uniq -c | sort -rn | head -10"
