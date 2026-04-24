use super::{create_temp_dir, ProcessError, ProcessResult};

/// Extract text from a PDF file.
///
/// Reads the PDF bytes, extracts selectable text via `pdf-extract`,
/// and normalises line endings to `\n`.
pub fn process(input_path: &str) -> Result<ProcessResult, ProcessError> {
    let bytes = std::fs::read(input_path)?;

    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| ProcessError::Pdf(format!("could not extract text: {}", e)))?;

    let normalised = text.replace("\r\n", "\n").replace('\r', "\n");

    let temp_dir = create_temp_dir()?;
    let output_path = temp_dir.join("output.txt");
    std::fs::write(&output_path, &normalised)?;

    let output_path_str = output_path
        .to_str()
        .ok_or(ProcessError::Validation(
            "could not convert output path to string".to_string(),
        ))?
        .to_string();

    let input_stem = std::path::Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let output_size = std::fs::metadata(&output_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(ProcessResult {
        output_path: output_path_str,
        output_name: format!("{}.txt", input_stem),
        output_mime: "text/plain".to_string(),
        output_size,
    })
}
