use crate::error::CliError;
use std::path::Path;

/// Reads a plain-text or Markdown file as UTF-8.
pub fn parse(file_path: &Path) -> Result<String, CliError> {
    std::fs::read_to_string(file_path).map_err(|source| CliError::FileRead {
        path: file_path.to_path_buf(),
        source,
    })
}
