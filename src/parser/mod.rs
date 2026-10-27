mod docx;
mod pdf;
mod txt;

use crate::error::CliError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The path that means "read from stdin".
pub const STDIN: &str = "-";

const SUPPORTED: [&str; 4] = ["txt", "md", "pdf", "docx"];

fn extension(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

/// True if `path` has an extension this tool can read.
pub fn is_supported(path: &Path) -> bool {
    SUPPORTED.contains(&extension(path).as_str())
}

/// Reads `file_path` and returns its text, choosing a parser by file extension.
/// A path of `-` reads plain text from stdin.
///
/// Errors:
/// - `UnsupportedFormat` if the extension is not .txt, .md, .pdf or .docx
/// - `FileRead` if the file is missing or cannot be read
/// - `EmptyFile` if the file is 0 bytes, or has no text after parsing
/// - `Parse` if a PDF or DOCX file is broken
pub fn extract_text(file_path: &Path) -> Result<String, CliError> {
    let text = if file_path == Path::new(STDIN) {
        io::read_to_string(io::stdin()).map_err(|source| CliError::FileRead {
            path: PathBuf::from(STDIN),
            source,
        })?
    } else {
        read_file(file_path)?
    };

    // Non-zero size but no words, e.g. only whitespace or a scanned PDF with no text layer
    if text.trim().is_empty() {
        return Err(CliError::EmptyFile(file_path.to_path_buf()));
    }

    Ok(text)
}

fn read_file(file_path: &Path) -> Result<String, CliError> {
    // Check the extension first: it needs no disk access
    let ext = extension(file_path);
    if !SUPPORTED.contains(&ext.as_str()) {
        return Err(CliError::UnsupportedFormat(ext));
    }

    let meta = fs::metadata(file_path).map_err(|source| CliError::FileRead {
        path: file_path.to_path_buf(),
        source,
    })?;

    // A 0-byte file is empty for every format; catch it before a parser fails on it
    if meta.len() == 0 {
        return Err(CliError::EmptyFile(file_path.to_path_buf()));
    }

    match ext.as_str() {
        "txt" | "md" => txt::parse(file_path),
        "pdf" => pdf::parse(file_path),
        "docx" => docx::parse(file_path),
        _ => unreachable!("extension was checked above"),
    }
}
