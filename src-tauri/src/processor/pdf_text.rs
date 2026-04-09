use super::{create_temp_dir, ProcessResult};

/// Extract text from a PDF file.
///
/// Reads the PDF bytes, extracts selectable text via `pdf-extract`,
/// and normalises line endings to `\n`.
pub fn process(input_path: &str) -> Result<ProcessResult, String> {
    let bytes = std::fs::read(input_path).map_err(|e| format!("could not read pdf file: {}", e))?;

    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| format!("could not extract text from pdf: {}", e))?;

    let normalised = text.replace("\r\n", "\n").replace('\r', "\n");

    let temp_dir = create_temp_dir()?;
    let output_path = temp_dir.join("output.txt");
    std::fs::write(&output_path, &normalised)
        .map_err(|e| format!("could not write output file: {}", e))?;

    let output_path_str = output_path
        .to_str()
        .ok_or("could not convert output path to string")?
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
