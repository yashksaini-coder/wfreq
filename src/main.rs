// The program runs as a CLI interface able to take file as input and return the top N most frequent words.
// Basic Process follows:-
//  - read a file, or from a directory containing the files
//  - run algorithm to count words, filtering, normalization
//  - Create statistics, total words parsed, unique word count, lexical diversity ratio
//  - show as proper block to visualize the count
//      - a heatmap correlation
//      - a ANSI table for CLI
//      - using a -- format flag for table, csv, plain,
// Set default value to N, can configure the top limit

use clap::Parser;
use std::collections::HashMap;
use std::error::Error;

/// A fast CLI tool to count the top N most frequent words in supported files.
#[derive(Parser, Debug)]
pub struct Cli {
    // A single filepath passed as input to CLI
    #[arg(required = true, num_args = 1)]
    file: std::path::PathBuf,

    // The top N frequent words to process and show
    #[arg(short, long, default_value_t = 10)]
    top: usize,
    // //The contents of the file in String
    // result: String
}

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

// fn main() {
//
//     let args = Cli::parse();
//     let content = std::fs::read_to_string(&args.file, ).expect("File can not read");
//     let top = args.top;
//     let words = top_n_words(&content, top);
//     println!("Path : {:?}", args.file);
//     println!("Content is {:?} ", content);
//     println!("The top {:?} words are: {:?}",args.top, words );
//
// }

// 1. Change the return type of main
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // 2. Replace .expect() with the ? operator
    // If read_to_string fails, the ? operator will immediately return the error from main
    let content = std::fs::read_to_string(&args.file)?;

    let top = args.top;
    let words = top_n_words(&content, top);

    println!("Path : {:?}", args.file);
    println!("The top {} words are: {:?}", args.top, words);

    // 3. Return Ok(()) at the end to indicate successful execution
    Ok(())
}
