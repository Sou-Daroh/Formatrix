pub mod csv;
pub mod image;
pub mod pdf_merge;
pub mod pdf_split;
pub mod pdf_text;

use serde::{Deserialize, Serialize};

/// Typed error enum for all processing operations.
/// Replaces raw `String` errors with traceable, categorised variants.
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] ::image::ImageError),

    #[error("CSV error: {0}")]
    Csv(#[from] ::csv::Error),

    #[error("PDF error: {0}")]
    Pdf(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("{0}")]
    Validation(String),
}

/// Result returned to the frontend after processing.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub output_path: String,
    pub output_name: String,
    pub output_mime: String,
    pub output_size: u64,
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
pub fn create_temp_dir() -> Result<std::path::PathBuf, ProcessError> {
    let id = uuid::Uuid::new_v4().to_string();
    let dir = std::env::temp_dir().join("formatrix").join(&id);
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
