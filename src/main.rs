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
