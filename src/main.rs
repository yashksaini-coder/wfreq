use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use wordfreq::cli::Cli;
use wordfreq::core::{STOP_WORDS, count_words, ignore_words, max_min_words, top_n};
use wordfreq::error::CliError;
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

    // if content.trim().is_empty() {
    //     eprintln!("Error: The file is completely empty.");
    // }
    if content.trim().is_empty() {
        return Err(CliError::EmptyFile(args.file.clone()).into());
    }

    let mut counts = count_words(&content);

    // Only unwrap what actually needs unwrapping (args.max_len)
    if let Some(max) = args.max_len {
        // Read args.min_len directly inside the closure
        counts.retain(|word, _| word.len() > args.min_len && word.len() < max);
    } else if args.min_len > 0 {
        // If only min_len was provided
        counts.retain(|word, _| word.len() > args.min_len);
    }

    // let stop_words: HashSet<&str> = STOP_WORDS.iter().copied().collect();
    //
    // let counts = if args.ignore_stop_words {
    //     ignore_words(counts, stop_words)
    // } else {
    //     counts
    // };
    if args.ignore_stop_words {
        let stop_words: HashSet<&str> = STOP_WORDS.iter().copied().collect();
        ignore_words(&mut counts, &stop_words);
    }

    let top = top_n(&counts, args.top);

    println!("Path: {}", args.file.display());
    for (i, (word, count)) in top.iter().enumerate() {
        println!("{:>2}. {:<15} {}", i + 1, word, count);
    }

    Ok(())
}
