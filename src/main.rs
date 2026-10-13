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
use wordfreq::core::top_n_words;

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
