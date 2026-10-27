use crate::error::CliError;
use pdf_extract::extract_text;
use std::path::Path;

// Main fn to parse the PDF files using external crate but keeping the style same through all parser
// extension and then mapping error to our own Cli::Error enum type
pub fn parse(file_path: &Path) -> Result<String, CliError> {
    extract_text(file_path).map_err(|e| CliError::Parse {
        path: file_path.to_path_buf(),
        reason: e.to_string(),
    })
}
