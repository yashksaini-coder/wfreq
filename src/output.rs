//! Writes the top-N results in the format chosen with `--format`.

use clap::ValueEnum;
use std::io::{self, Write};

/// Output formats for the results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    /// Numbered list (default)
    Plain,
    /// Box table with a bar for each count
    Table,
    /// rank,word,count with a header row, for spreadsheets and scripts
    Csv,
}

/// Width of the longest bar in table output, in characters.
const BAR_WIDTH: usize = 20;

/// Writes `top` to `out` in the given format.
pub fn write_top(out: &mut impl Write, top: &[(String, usize)], format: Format) -> io::Result<()> {
    match format {
        Format::Plain => write_plain(out, top),
        Format::Table => write_table(out, top),
        Format::Csv => write_csv(out, top),
    }
}

fn write_plain(out: &mut impl Write, top: &[(String, usize)]) -> io::Result<()> {
    for (i, (word, count)) in top.iter().enumerate() {
        writeln!(out, "{:>2}. {:<15} {}", i + 1, word, count)?;
    }
    Ok(())
}

fn write_csv(out: &mut impl Write, top: &[(String, usize)]) -> io::Result<()> {
    writeln!(out, "rank,word,count")?;
    for (i, (word, count)) in top.iter().enumerate() {
        writeln!(out, "{},{},{}", i + 1, csv_field(word), count)?;
    }
    Ok(())
}

/// Quotes a CSV field when it contains a comma, quote or newline (RFC 4180).
fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn write_table(out: &mut impl Write, top: &[(String, usize)]) -> io::Result<()> {
    // Column widths in characters (not bytes), so words like "café" line up
    let rank_w = top.len().to_string().len();
    let word_w = top
        .iter()
        .map(|(w, _)| w.chars().count())
        .max()
        .unwrap_or(0)
        .max(4);
    let count_w = top
        .iter()
        .map(|(_, c)| c.to_string().len())
        .max()
        .unwrap_or(0)
        .max(5);
    // top is sorted, so the first count is the largest; max(1) avoids dividing by zero
    let max_count = top.first().map_or(1, |(_, c)| *c).max(1);

    let border = |left: &str, mid: &str, right: &str| {
        let seg = |w: usize| "─".repeat(w + 2);
        format!(
            "{left}{}{mid}{}{mid}{}{mid}{}{right}",
            seg(rank_w),
            seg(word_w),
            seg(count_w),
            seg(BAR_WIDTH)
        )
    };

    writeln!(out, "{}", border("┌", "┬", "┐"))?;
    writeln!(
        out,
        "│ {:>rank_w$} │ {:<word_w$} │ {:>count_w$} │ {:<BAR_WIDTH$} │",
        "#", "word", "count", ""
    )?;
    writeln!(out, "{}", border("├", "┼", "┤"))?;
    for (i, (word, count)) in top.iter().enumerate() {
        let bar = "█".repeat(count * BAR_WIDTH / max_count);
        writeln!(
            out,
            "│ {:>rank_w$} │ {:<word_w$} │ {:>count_w$} │ {:<BAR_WIDTH$} │",
            i + 1,
            word,
            count,
            bar
        )?;
    }
    writeln!(out, "{}", border("└", "┴", "┘"))?;
    Ok(())
}
