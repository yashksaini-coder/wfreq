use crate::output::Format;
use clap::Parser;
use std::path::PathBuf;

/// A fast CLI tool to count the top N most frequent words in supported files.

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Files or directories to read. Use - to read text from stdin
    #[arg(required = true)]
    pub files: Vec<PathBuf>,

    /// Also read files in subdirectories when a directory is given
    #[arg(short, long)]
    pub recursive: bool,

    /// Output format for the results
    #[arg(short, long, value_enum, default_value_t = Format::Plain)]
    pub format: Format,

    /// Number of top frequent words to display
    #[arg(short = 'n', long, default_value_t = 10)]
    pub top: usize,

    /// Make the word counting case-sensitive
    #[arg(short = 'i', long = "case-sensitive", default_value_t = false)]
    pub case_sensitive: bool,

    /// Ignore common stop words (e.g., 'the', 'is', 'and')
    #[arg(long, default_value_t = false)]
    pub ignore_stop_words: bool,

    /// Path to a custom file containing words to exclude (one per line)
    #[arg(short = 'x', long)]
    pub exclude: Option<PathBuf>,

    /// Minimum length of words to consider
    #[arg(long, default_value_t = 1)]
    pub min_len: usize,

    /// Maximum length of words to consider
    #[arg(long)]
    pub max_len: Option<usize>,

    /// Show execution time and processing statistics
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
