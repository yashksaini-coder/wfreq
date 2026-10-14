use std::path::Path;
use anyhow::{Context, Error};

pub fn parse(file_path: &Path) -> Result<String, Error>{
    std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))
    
}   
