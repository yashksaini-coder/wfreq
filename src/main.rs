use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use std::time::Instant;
use wordfreq::cli::Cli;
use wordfreq::core::{build_ignore_set, count_words, filter_by_length, merge_counts, stats, top_n};
use wordfreq::error::CliError;
use wordfreq::input::collect_files;
use wordfreq::output::{Format, write_top};
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
    let start_time = Instant::now();

    // Check the flags before doing any file work
    if let Some(max) = args.max_len
        && args.min_len > max
    {
        return Err(CliError::InvalidRange {
            min: args.min_len,
            max,
        }
        .into());
    }

    let files = collect_files(&args.files, args.recursive)?;

    // Count each file and add it to one combined map
    let mut counts = HashMap::new();
    let mut files_read = 0;
    for path in &files {
        match parser::extract_text(path) {
            Ok(text) => {
                merge_counts(&mut counts, count_words(&text, args.case_sensitive));
                files_read += 1;
            }
            // With several files, one bad file should not stop the others
            Err(e) if files.len() > 1 => eprintln!("warning: {e} (skipped)"),
            Err(e) => return Err(e.into()),
        }
    }
    if files_read == 0 {
        return Err(CliError::NoInput.into());
    }

    filter_by_length(&mut counts, args.min_len, args.max_len);

    let ignore_set = build_ignore_set(args.exclude.as_deref(), args.ignore_stop_words)?;
    if !ignore_set.is_empty() {
        counts.retain(|word, _| !ignore_set.contains(word.as_str()));
    }

    let top = top_n(&counts, args.top);

    let mut out = io::BufWriter::new(io::stdout().lock());
    // CSV output must contain only data rows, so it gets no header line
    if args.format != Format::Csv {
        if files.len() == 1 {
            writeln!(out, "Path: {}", files[0].display())?;
        } else {
            writeln!(out, "Files: {files_read}")?;
        }
    }
    write_top(&mut out, &top, args.format)?;

    if args.verbose {
        let s = stats(&counts);
        writeln!(out, "\n--- Processing Statistics ---")?;
        writeln!(out, "Files read           : {files_read}")?;
        writeln!(out, "Execution time       : {:?}", start_time.elapsed())?;
        writeln!(out, "Total Words          : {}", s.total)?;
        writeln!(out, "Unique Words         : {}", s.unique)?;
        writeln!(out, "Diversity Ratio      : {}", s.diversity)?;
    }

    Ok(())
}
