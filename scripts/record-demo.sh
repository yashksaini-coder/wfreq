#!/usr/bin/env bash
# Records the README demo: types real commands and runs the real wfreq binary.
#
# Needs asciinema (cargo install --locked asciinema) and agg (cargo install --locked agg).
# Run from the repo root: ./scripts/record-demo.sh
# Output: docs/images/demo.gif (and target/demo.cast)
set -euo pipefail

if [ "${1:-}" != "--play" ]; then
    cargo build --release --quiet
    asciinema rec --overwrite --window-size 100x30 --idle-time-limit 2 \
        --command "$0 --play" target/demo.cast
    agg --font-family "JetBrainsMono Nerd Font" --font-size 16 --theme dracula \
        --last-frame-duration 4 target/demo.cast docs/images/demo.gif
    echo "wrote docs/images/demo.gif"
    exit 0
fi

# --- the scripted session (runs inside asciinema) ---
export PATH="$PWD/target/release:$PATH"

type_run() {
    printf '\033[1;32m$\033[0m '
    local cmd="$1"
    for ((i = 0; i < ${#cmd}; i++)); do
        printf '%s' "${cmd:i:1}"
        sleep 0.035
    done
    sleep 0.5
    printf '\n'
    eval "$cmd" || true
    sleep "${2:-2}"
    printf '\n'
}

clear
type_run "wfreq files/cp.txt -n 5"
type_run "wfreq files/dsa_patterns.docx --ignore-stop-words --min-len 4 --max-len 10 -n 6 -f table" 3
type_run "echo 'to be or not to be' | wfreq - -n 3"
type_run "wfreq files -r -n 4 -f csv"
type_run "wfreq files/cp.txt -n 3 -v" 2.5
type_run "wfreq missing.txt" 3
