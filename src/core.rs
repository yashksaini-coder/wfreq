//! Word counting, filtering and statistics.
//!
//! Everything here works on text and maps only, except [`build_ignore_set`],
//! which reads the `--exclude` file.
//!
//! Flow: `count_words` reads the text once into a frequency map; every other
//! function here borrows that map, so the text is never scanned twice.

use crate::error::CliError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
// use std::{fs, io};
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
/// digits, and apostrophes is removed, and unless `case_sensitive` is true the
/// result is lowercased (`"Hello,"` → `"hello"`). Tokens that end up empty
/// (e.g. `"--"`) are skipped.
///
/// ```
/// let counts = wfreq::core::count_words("Hello, hello world!", false);
/// assert_eq!(counts["hello"], 2);
/// assert_eq!(counts["world"], 1);
/// ```
///
/// Returns a map of `word → occurrence count`. The map has no ordering;
/// use [`top_n`] for a ranked list.
pub fn count_words(content: &str, case_sensitive: bool) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in content.split_whitespace() {
        let mut clean_word: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>();

        if clean_word.is_empty() {
            continue;
        }

        if !case_sensitive {
            clean_word = clean_word.to_lowercase();
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

// /// The main fn to pass the ignore words as a Hash set in order to handle duplicate items
// /// Returns a &str borrow from the `map`, so the map value don't get used and result in a borrow
// /// checker error.
// pub fn ignore_words(map: HashMap<String, usize>, list: HashSet<&str>) -> HashMap<String, usize> {
//     map.into_iter()
//         .filter(|(word, _)| !list.contains(word.as_str()))
//         .collect()
// }

/// Removes every word in `list` from `map`, in place.
pub fn ignore_words(map: &mut HashMap<String, usize>, list: &HashSet<&str>) {
    map.retain(|word, _| !list.contains(word.as_str()));
}

/// Common words removed by `--ignore-stop-words`.
pub const STOP_WORDS: &[&str] = &["the", "for", "on", "in", "and", "a", "is", "to"];

// This is the idiomatic way of working the Min and Max len of words to return.
// But a more better approach is to use the `retain` if we  want to avoid destroying the old HashMap and allocating memory for a completely new one
// It filters the dictionary in-place.

// /// This fn will run a value comparison on the given hasmap of words with a n argument as input
// /// Returns a &str borrow from the `map`, again to avoid borrow checker error.
// pub fn max_min_words(
//     map: HashMap<String, usize>,
//     max: usize,
//     min: usize,
// ) -> HashMap<String, usize> {
//     map.into_iter()
//         .filter(|(word, _)| max > word.len() && word.len() > min)
//         .collect()
// }

/// Keeps only words whose length in characters is within `min..=max`, in place.
/// `max = None` means no upper limit.
pub fn filter_by_length(map: &mut HashMap<String, usize>, min: usize, max: Option<usize>) {
    map.retain(|word, _| {
        let len = word.chars().count();
        len >= min && max.is_none_or(|m| len <= m)
    });
}

/// Builds the set of words to remove from the counts.
///
/// Adds [`STOP_WORDS`] when `include_defaults` is true, and every non-empty
/// line of `exclude_file` (trimmed and lowercased) when a path is given.
///
/// Returns [`CliError::FileRead`] if the exclude file cannot be read.
pub fn build_ignore_set(
    exclude_file: Option<&Path>,
    include_defaults: bool,
    // ) -> Result<HashSet<String>, io::Error> {
) -> Result<HashSet<String>, CliError> {
    let mut ignore_set: HashSet<String> = HashSet::new();

    // 1. Add the original built-in defaults flag of `include_stop_words`
    if include_defaults {
        for &word in STOP_WORDS {
            // .to_string() allocates memory to convert the &str into an owned String
            ignore_set.insert(word.to_string());
        }
    }

    // 2. Reading words from the custom file if a path was provided
    if let Some(path) = exclude_file {
        // Read entire file into a String
        // let file_content = fs::read_to_string(path)?;
        let file_content = fs::read_to_string(path).map_err(|source| CliError::FileRead {
            path: path.to_path_buf(),
            source,
        })?;

        // .lines() can automatically splits the text by newlines (\n or \r\n)
        for line in file_content.lines() {
            let clean_word = line.trim().to_lowercase(); // Sanitize the input

            if !clean_word.is_empty() {
                ignore_set.insert(clean_word);
            }
        }
    }

    Ok(ignore_set)
}

/// Adds the counts in `from` into `into`, so several files can be counted together.
pub fn merge_counts(into: &mut HashMap<String, usize>, from: HashMap<String, usize>) {
    for (word, count) in from {
        *into.entry(word).or_insert(0) += count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, usize)]) -> HashMap<String, usize> {
        pairs.iter().map(|(w, c)| (w.to_string(), *c)).collect()
    }

    #[test]
    fn counts_repeated_words() {
        let counts = count_words("a a b", false);
        assert_eq!(counts, map(&[("a", 2), ("b", 1)]));
    }

    #[test]
    fn lowercases_unless_case_sensitive() {
        assert_eq!(count_words("The the THE", false), map(&[("the", 3)]));
        assert_eq!(count_words("The the", true), map(&[("The", 1), ("the", 1)]));
    }

    #[test]
    fn strips_punctuation_but_keeps_apostrophes() {
        let counts = count_words("hello, hello! don't", false);
        assert_eq!(counts, map(&[("hello", 2), ("don't", 1)]));
    }

    #[test]
    fn skips_tokens_that_are_only_punctuation() {
        assert!(count_words("-- ... !!", false).is_empty());
        assert!(count_words("", false).is_empty());
    }

    #[test]
    fn top_n_sorts_by_count_then_word() {
        let counts = map(&[("b", 1), ("a", 1), ("c", 5)]);
        let top = top_n(&counts, 3);
        assert_eq!(
            top,
            vec![
                ("c".to_string(), 5),
                ("a".to_string(), 1),
                ("b".to_string(), 1)
            ]
        );
    }

    #[test]
    fn top_n_handles_large_and_zero_n() {
        let counts = map(&[("a", 1), ("b", 2)]);
        assert_eq!(top_n(&counts, 100).len(), 2);
        assert!(top_n(&counts, 0).is_empty());
    }

    #[test]
    fn exactly_one_returns_words_seen_once() {
        let counts = map(&[("a", 1), ("b", 2), ("c", 1)]);
        let mut once = exactly_one(&counts);
        once.sort();
        assert_eq!(once, vec!["a", "c"]);
    }

    #[test]
    fn stats_on_empty_map_has_zero_diversity() {
        let s = stats(&HashMap::new());
        assert_eq!((s.total, s.unique), (0, 0));
        assert_eq!(s.diversity, 0.0);
    }

    #[test]
    fn stats_counts_total_unique_and_diversity() {
        let s = stats(&map(&[("a", 2), ("b", 2)]));
        assert_eq!((s.total, s.unique), (4, 2));
        assert_eq!(s.diversity, 0.5);
    }

    #[test]
    fn length_filter_bounds_are_inclusive() {
        let mut counts = map(&[("ab", 1), ("abc", 1), ("abcd", 1), ("abcde", 1)]);
        filter_by_length(&mut counts, 3, Some(4));
        assert_eq!(counts, map(&[("abc", 1), ("abcd", 1)]));
    }

    #[test]
    fn length_filter_counts_chars_not_bytes() {
        // "café" is 4 characters but 5 bytes
        let mut counts = map(&[("café", 1)]);
        filter_by_length(&mut counts, 1, Some(4));
        assert_eq!(counts, map(&[("café", 1)]));
    }

    #[test]
    fn length_filter_without_max_has_no_upper_limit() {
        let mut counts = map(&[("a", 1), ("abcdefghijklmnop", 1)]);
        filter_by_length(&mut counts, 2, None);
        assert_eq!(counts, map(&[("abcdefghijklmnop", 1)]));
    }

    #[test]
    fn ignore_words_removes_listed_words() {
        let mut counts = map(&[("the", 3), ("cat", 1)]);
        ignore_words(&mut counts, &HashSet::from(["the"]));
        assert_eq!(counts, map(&[("cat", 1)]));
    }

    #[test]
    fn merge_counts_adds_matching_words() {
        let mut total = map(&[("a", 1), ("b", 2)]);
        merge_counts(&mut total, map(&[("b", 3), ("c", 1)]));
        assert_eq!(total, map(&[("a", 1), ("b", 5), ("c", 1)]));
    }

    #[test]
    fn ignore_set_contains_stop_words_when_asked() {
        let set = build_ignore_set(None, true).unwrap();
        assert_eq!(set.len(), STOP_WORDS.len());
        assert!(build_ignore_set(None, false).unwrap().is_empty());
    }

    #[test]
    fn ignore_set_reports_missing_exclude_file() {
        let err = build_ignore_set(Some(Path::new("no/such/file.txt")), false).unwrap_err();
        assert!(matches!(err, CliError::FileRead { .. }));
    }
}
