use crate::error::CliError;
use docx_lite::extract_text;
use std::path::Path;
pub fn parse(file_path: &Path) -> Result<String, CliError> {
    extract_text(file_path).map_err(|e| CliError::ParseError(e.to_string()))
}
