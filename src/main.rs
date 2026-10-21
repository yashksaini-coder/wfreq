use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use wordfreq::cli::Cli;
use wordfreq::core::{STOP_WORDS, count_words, ignore_words, max_n_words, top_n};
use wordfreq::parser;

fn main() {
    // Print errors with Display (our #[error] messages) instead of main's default Debug
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let content = parser::extract_text(&args.file)?;

    if content.trim().is_empty() {
        eprintln!("Error: The file is completely empty.");
    }

    let mut counts = count_words(&content);

    if args.max_len > 0 {
        counts = max_n_words(counts, args.max_len);
    }

    let stop_words: HashSet<&str> = STOP_WORDS.iter().copied().collect();

    let counts = if args.ignore_stop_words {
        ignore_words(counts, stop_words)
    } else {
        counts
    };

    let top = top_n(&counts, args.top);

    println!("Path: {}", args.file.display());
    for (i, (word, count)) in top.iter().enumerate() {
        println!("{:>2}. {:<15} {}", i + 1, word, count);
    }

    Ok(())
}
