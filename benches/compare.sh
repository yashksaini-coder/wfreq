#!/usr/bin/env bash
# Compare wfreq against the classic Unix word-count pipeline with hyperfine,
# at 8 input sizes, then draw docs/images/bench-{light,dark}.svg.
#
# Needs: pdftotext (poppler-utils), hyperfine 2.x and python3.
# Run from the repo root: ./benches/compare.sh
set -euo pipefail
export LC_ALL=C   # byte-wise sort/tr: the fastest setting for the Unix pipeline

mkdir -p target/bench
[ -f target/bench/handbook.txt ] ||
    pdftotext files/Competitive-Programmer-Handbook.pdf target/bench/handbook.txt

# Real English text (the Competitive Programmer's Handbook, 87k words),
# repeated 10, 20, ... 80 times: 4 MB to 35 MB.
sizes=10,20,30,40,50,60,70,80
for n in ${sizes//,/ }; do
    f=target/bench/corpus-$n.txt
    [ -f "$f" ] || for _ in $(seq "$n"); do cat target/bench/handbook.txt; done > "$f"
done

cargo build --release --quiet

# Same job in both: lowercase, split on anything that is not a letter, digit or
# apostrophe, count, print the top 10. hyperfine also records each run's peak
# memory (for the pipeline: its biggest process, which is sort).
bench() {
    hyperfine --shell=bash --warmup 1 --runs 10 -L n "$sizes" \
        --export-json "target/bench/$1.json" "$2"
}
bench wfreq './target/release/wfreq target/bench/corpus-{n}.txt -n 10'
bench pipeline "tr -cs \"[:alnum:]'\" '\n' < target/bench/corpus-{n}.txt | tr '[:upper:]' '[:lower:]' | sort | uniq -c | sort -rn | head -10"

python3 benches/plot.py target/bench docs/images
