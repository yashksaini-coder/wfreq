use std::path::Path;
use anyhow::{bail, Result};

pub fn parse(_file_path: &Path) -> Result<String> {
    bail!("Docx support not implemented yet")
}
