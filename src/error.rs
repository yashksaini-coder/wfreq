use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("cannot read {}: {source}", .path.display())]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("File can not be parsed: {0}")]
    ParseError(String),

    #[error("File {0} is completely empty")]
    EmptyFile(PathBuf),

    #[error("unsupported file format '.{0}' (supported: .txt, .md)")]
    UnsupportedFormat(String),
}
