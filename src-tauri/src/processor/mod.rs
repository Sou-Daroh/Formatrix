pub mod csv;
pub mod image;
pub mod pdf_merge;
pub mod pdf_text;

use serde::{Deserialize, Serialize};

/// Result returned to the frontend after processing.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub output_path: String,
    pub output_name: String,
    pub output_mime: String,
}

/// Options for image resize and compress.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageOptions {
    pub width: u32,
    pub height: u32,
    pub quality: u8,
    pub format: String,
}

/// Options for CSV to JSON conversion.
#[derive(Debug, Serialize, Deserialize)]
pub struct CsvOptions {
    pub pretty: bool,
}

/// Options for PDF split.
#[derive(Debug, Serialize, Deserialize)]
pub struct PdfSplitOptions {
    pub pages: String,
}

/// Payload emitted as a progress event during processing.
#[derive(Debug, Clone, Serialize)]
pub struct ProgressPayload {
    pub percent: u8,
    pub stage: String,
}

/// Create a UUID-named subdirectory in the OS temp folder for output files.
pub fn create_temp_dir() -> Result<std::path::PathBuf, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let dir = std::env::temp_dir().join("formatrix").join(&id);
    std::fs::create_dir_all(&dir).map_err(|e| format!("could not create temp directory: {}", e))?;
    Ok(dir)
}
