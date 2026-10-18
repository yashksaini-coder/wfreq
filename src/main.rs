use clap::Parser;
use std::error::Error;
use wordfreq::cli::Cli;
use wordfreq::core::{count_words, top_n};
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

    let counts = count_words(&content);
    let top = top_n(&counts, args.top);

    if content.trim().is_empty() {
        eprintln!("Error: The file is completely empty.");
    }

    println!("Path: {}", args.file.display());
    for (i, (word, count)) in top.iter().enumerate() {
        println!("{:>2}. {:<15} {}", i + 1, word, count);
    }

    Ok(())
}
