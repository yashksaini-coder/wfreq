//! Use wfreq as a library: read one file, drop stop words, print the top 5.
//!
//! Run it with:
//!
//! ```text
//! cargo run --example top_words -- files/cp.txt
//! ```

use std::path::PathBuf;
use wfreq::core::{build_ignore_set, count_words, top_n};
use wfreq::output::{Format, write_top};
use wfreq::parser::extract_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: top_words <FILE>")?;

    let text = extract_text(&path)?;
    let mut counts = count_words(&text, false);

    let stop_words = build_ignore_set(None, true)?;
    counts.retain(|word, _| !stop_words.contains(word));

    let top = top_n(&counts, 5);
    write_top(&mut std::io::stdout(), &top, Format::Table)?;
    Ok(())
}
