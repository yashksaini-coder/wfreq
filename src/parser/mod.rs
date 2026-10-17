pub mod txt;

use crate::error::CliError;
use std::path::Path;

/// Reads `file_path` and returns its text, choosing a parser by file extension.
pub fn extract_text(file_path: &Path) -> Result<String, CliError> {
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "txt" | "md" => txt::parse(file_path),
        _ => Err(CliError::UnsupportedFormat(ext)),
    }
}
