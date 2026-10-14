use std::collections::HashMap;

// 1. Define the struct to hold the output
#[derive(Debug)]
pub struct Stats {
    pub total: usize,
    pub unique: usize,
    pub diversity: f64,
}

pub fn count_words(content: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in content.split_whitespace() {
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>()
            .to_lowercase();

        if clean_word.is_empty(){
            continue;
        }
        *map.entry(clean_word).or_insert(0) +=1;
    }

    map
}


pub fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    // 1. Collect references to the map's keys and values into a Vector
    let mut sorted: Vec<(&String, &usize)> = map.iter().collect();

    // 2. Sort: Descending by count (b.1 vs a.1), then Ascending by word (a.0 vs b.0)
    sorted.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    // 3. Take the first 'n', and clone the borrowed data into owned types to return
    sorted
        .into_iter()
        .take(n)
        .map(|(word, count)| (word.clone(), *count))
        .collect()

}

pub fn exactly_one(map: &HashMap<String, usize>) -> Vec<&str> {
    map.iter().filter_map(|(word, count) | (*count ==1).then_some(word.as_str()))
    .collect()
}

pub fn stats(map: &HashMap<String, usize>) -> Stats{
    let unique = map.len();

    let total: usize = map.values().sum();

    // 3. Cast to f64 for floating-point division, handling the edge case of an empty file
    let diversity = if total > 0 {
        (unique as f64) / (total as f64)
    } else {
        0.0
    };

    Stats { total, unique, diversity }
}