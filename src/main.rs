use ::std::io;
use clap::Parser;
use std::error::Error;
use std::io::Write;
use std::time::Instant;
use wordfreq::cli::Cli;
use wordfreq::core::{build_ignore_set, count_words, filter_by_length, stats, top_n};
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
    let cout = io::stdout();
    let mut handle = io::BufWriter::new(cout);
    let args = Cli::parse();

    // starting the clock
    let start_time = Instant::now();
    let content = parser::extract_text(&args.file)?;

    // if content.trim().is_empty() {
    //     eprintln!("Error: The file is completely empty.");
    // }
    if content.trim().is_empty() {
        return Err(CliError::EmptyFile(args.file.clone()).into());
    }

    if let Some(max) = args.max_len
        && args.min_len > max
    {
        return Err(CliError::InvalidRange {
            min: args.min_len,
            max,
        }
        .into());
    }

    let mut counts = count_words(&content, args.case_sensitive);

    // // Only unwrap what actually needs unwrapping (args.max_len)
    // if let Some(max) = args.max_len {
    //     // Read args.min_len directly inside the closure
    //     counts.retain(|word, _| word.len() > args.min_len && word.len() < max);
    // } else if args.min_len > 0 {
    //     // If only min_len was provided
    //     counts.retain(|word, _| word.len() > args.min_len);
    // }
    filter_by_length(&mut counts, args.min_len, args.max_len);

    let ignore_set = build_ignore_set(args.exclude.as_deref(), args.ignore_stop_words)?;
    if !ignore_set.is_empty() {
        counts.retain(|word, _| !ignore_set.contains(word.as_str()));
    }
    let top = top_n(&counts, args.top);

    // println!("Path: {}", args.file.display());
    // Moving to writeln as it it 8x faster than println macro,
    // and is best practice to use for everything related to output order ensuring sequential order.
    writeln!(handle, "Path: {}", args.file.display())?;
    handle.flush()?;
    for (i, (word, count)) in top.iter().enumerate() {
        println!("{:>2}. {:<15} {}", i + 1, word, count);
    }

    if args.verbose {
        let duration = start_time.elapsed();
        writeln!(handle, "\n--- Processing Statistics ---")?;
        writeln!(handle, "Unique words tracked : {}", counts.len())?;
        writeln!(handle, "Execution time       : {:?}", duration)?;
        writeln!(handle, "Stats                : {:?}", stats(&counts))?;
    }

    Ok(())
}
