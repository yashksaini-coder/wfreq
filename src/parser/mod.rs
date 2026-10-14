pub mod txt;
pub mod pdf;
pub mod docx;

use std::path::Path;
use anyhow::{Result, bail};

// 2. Create a unified public function for main.rs to call.
// This hides the complexity of choosing the right parser from the rest of your app.
pub fn extract_text(file_path: &Path) -> Result<String> {
    // Safely extract the file extension
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 3. Route the file to the correct specialized parser
    match extension.as_str() {
        "txt" | "md" => txt::parse(file_path),
        "pdf" => pdf::parse(file_path),
        "docx" => docx::parse(file_path),
        _ => bail!("Unsupported file extension: .{}", extension), // Returns an anyhow error
    }
}