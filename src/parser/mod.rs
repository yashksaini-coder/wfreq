mod docx;
mod pdf;
mod txt;

use crate::error::CliError;
use std::fs;
use std::path::Path;

/// Reads `file_path` and returns its text, choosing a parser by file extension.
///
/// Errors:
/// - `UnsupportedFormat` if the extension is not .txt, .md, .pdf or .docx
/// - `FileRead` if the file is missing or cannot be read
/// - `EmptyFile` if the file is 0 bytes, or has no text after parsing
/// - `ParseError` if a PDF or DOCX file is broken
pub fn extract_text(file_path: &Path) -> Result<String, CliError> {
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Check the extension first: it needs no disk access
    if !matches!(ext.as_str(), "txt" | "md" | "pdf" | "docx") {
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

    let text = match ext.as_str() {
        "txt" | "md" => txt::parse(file_path)?,
        "pdf" => pdf::parse(file_path)?,
        "docx" => docx::parse(file_path)?,
        _ => unreachable!("extension was checked above"),
    };

    // Non-zero size but no words, e.g. only whitespace or a scanned PDF with no text layer
    if text.trim().is_empty() {
        return Err(CliError::EmptyFile(file_path.to_path_buf()));
    }

    Ok(text)
}
