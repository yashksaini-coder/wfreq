use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("File not found or cannot read: {0}")]
    FileReadError(PathBuf),

    #[error("File can not be parsed: {0}")]
    ParseError(String),

    #[error("File {0} is completely empty")]
    EmptyFile(PathBuf),

    #[error("Unsupported file format : {0}. The supported formats are .txt, .md, .pdf, .docx")]
    UnsupportedFormat(String),
}
