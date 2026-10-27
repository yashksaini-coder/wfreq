//! The error type used across the crate.

use std::path::PathBuf;
use thiserror::Error;

/// Every error the tool can report. The `Display` text is what users see.
#[derive(Debug, Error)]
pub enum CliError {
    /// A file or folder could not be read (missing, no permission, not UTF-8, ...).
    #[error("cannot read {}: {source}", .path.display())]
    FileRead {
        /// The file or folder that failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A PDF or DOCX file could not be parsed.
    #[error("cannot parse {}: {reason}", .path.display())]
    Parse {
        /// The file that failed.
        path: PathBuf,
        /// The parser's error message.
        reason: String,
    },

    /// The file is 0 bytes or contains only whitespace.
    #[error("File {0} is completely empty")]
    EmptyFile(PathBuf),

    /// The file extension is not .txt, .md, .pdf or .docx.
    #[error("unsupported file format '.{0}' (supported: .txt, .md, .pdf, .docx)")]
    UnsupportedFormat(String),

    /// `--min-len` is greater than `--max-len`.
    #[error("--min-len ({min}) is greater than --max-len ({max})")]
    InvalidRange {
        /// The `--min-len` value.
        min: usize,
        /// The `--max-len` value.
        max: usize,
    },

    /// Every given file failed, so there is nothing to count.
    #[error("none of the given files could be read")]
    NoInput,
}
