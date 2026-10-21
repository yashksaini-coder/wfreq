//! Word counting and statistics. Pure functions only, no file I/O.
//!
//! Flow: `count_words` reads the text once into a frequency map; every other
//! function here borrows that map, so the text is never scanned twice.

use std::collections::{HashMap, HashSet};
/// Summary statistics computed from a word-frequency map.
#[derive(Debug)]
pub struct Stats {
    /// Total number of words counted (every occurrence).
    pub total: usize,
    /// Number of distinct words (vocabulary size).
    pub unique: usize,
    /// Lexical diversity (type-token ratio): `unique / total`, in `0.0..=1.0`.
    /// Closer to 1.0 means fewer repeated words. `0.0` for empty input.
    pub diversity: f64,
}

/// Counts how many times each word appears in `content`.
///
/// Words are split on whitespace, then normalized: everything except letters,
/// digits, and apostrophes is removed, and the result is lowercased
/// (`"Hello,"` → `"hello"`). Tokens that end up empty (e.g. `"--"`) are skipped.
///
/// Returns a map of `word → occurrence count`. The map has no ordering;
/// use [`top_n`] for a ranked list.
pub fn count_words(content: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in content.split_whitespace() {
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>()
            .to_lowercase();

        if clean_word.is_empty() {
            continue;
        }
        *map.entry(clean_word).or_insert(0) += 1;
    }

    map
}

/// Returns the `n` most frequent words, highest count first.
///
/// Ties are broken alphabetically so the output is the same on every run.
/// If `n` exceeds the number of distinct words, all words are returned;
/// `n == 0` returns an empty `Vec`.
///
/// Returns owned `(word, count)` pairs; only the `n` returned words are cloned.
pub fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    // Sort borrowed entries so nothing is cloned until we know what we keep
    let mut sorted: Vec<(&String, &usize)> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    sorted
        .into_iter()
        .take(n)
        .map(|(word, count)| (word.clone(), *count))
        .collect()
}

/// Returns the words that occur exactly once (hapax legomena).
///
/// The returned `&str`s borrow from `map`, so `map` must outlive the result.
/// Order is unspecified (it follows `HashMap` iteration); sort it before display.
pub fn exactly_one(map: &HashMap<String, usize>) -> Vec<&str> {
    map.iter()
        .filter_map(|(word, count)| (*count == 1).then_some(word.as_str()))
        .collect()
}

/// Computes total words, distinct words, and lexical diversity for `map`.
///
/// Returns a [`Stats`]. Safe on an empty map: diversity is `0.0` instead of
/// dividing by zero.
pub fn stats(map: &HashMap<String, usize>) -> Stats {
    let unique = map.len();
    let total: usize = map.values().sum();

    let diversity = if total > 0 {
        (unique as f64) / (total as f64)
    } else {
        0.0
    };

    Stats {
        total,
        unique,
        diversity,
    }
}

/// The main fn to pass the ignore words as a Hash set in order to handle duplicate items
/// Returns a &str borrow from the `map`, so the map value don't get used and result in a borrow
/// checker error.
pub fn ignore_words(map: HashMap<String, usize>, list: HashSet<&str>) -> HashMap<String, usize> {
    map.into_iter()
        .filter(|(word, _)| !list.contains(word.as_str()))
        .collect()
}

pub const STOP_WORDS: &[&str] = &["the", "for", "on", "in", "and", "a", "is", "to"];

// This is the idiomatic way of working the Min and Max len of words to return.
// But a more better approach is to use the `retain` if we  want to avoid destroying the old HashMap and allocating memory for a completely new one
// It filters the dictionary in-place.

/// This fn will run a value comparison on the given hasmap of words with a n argument as input
/// Returns a &str borrow from the `map`, again to avoid borrow checker error.
pub fn max_n_words(map: HashMap<String, usize>, n: usize) -> HashMap<String, usize> {
    map.into_iter().filter(|(word, _)| word.len() > n).collect()
}
