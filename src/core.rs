use std::collections::HashMap;

pub fn top_n_words(content: &str, top: usize) -> Vec<(String, usize)> {
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for word in content.split_whitespace() {
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>()
            .to_lowercase();

        // "--" or "..." becomes "" after filtering; skip it
        if clean_word.is_empty() {
            continue;
        }

        *word_counts.entry(clean_word).or_insert(0) += 1;
    }

    let mut sorted: Vec<(String, usize)> = word_counts.into_iter().collect();
    // Highest count first; ties broken alphabetically so output is stable
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    sorted.truncate(top);
    sorted
}

