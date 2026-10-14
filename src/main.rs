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
use std::error::Error;
use wordfreq::cli::Cli;
use wordfreq::core::{count_words, top_n};
use wordfreq::parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let content = parser::extract_text(&args.file)?;

    let counts = count_words(&content);
    let top = top_n(&counts, args.top);

    println!("Path : {:?}", args.file.display());
    for (i, (word, count)) in top.iter().enumerate() {
        println!("{:>2}. {:<15} {}", i + 1, word, count);
    }

    // 3. Return Ok(()) at the end to indicate successful execution
    Ok(())
}
