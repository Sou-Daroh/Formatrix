use super::{create_temp_dir, CsvOptions, ProcessError, ProcessResult};

/// Convert a CSV file to JSON.
///
/// Each row becomes a JSON object with keys from the CSV header.
/// All values are strings -- no type inference is performed.
/// Missing fields (rows shorter than the header) become empty strings.
pub fn process(input_path: &str, options: &CsvOptions) -> Result<ProcessResult, ProcessError> {
    let mut reader = csv::Reader::from_path(input_path)?;

    let headers = reader
        .headers()?
        .iter()
        .map(|h| h.trim().to_string())
        .collect::<Vec<String>>();

    let mut records: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut map = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            let value = record.get(i).unwrap_or("").to_string();
            map.insert(header.clone(), serde_json::Value::String(value));
        }
        records.push(map);
    }

    let json = if options.pretty {
        serde_json::to_string_pretty(&records)?
    } else {
        serde_json::to_string(&records)?
    };

    let temp_dir = create_temp_dir()?;
    let output_path = temp_dir.join("output.json");
    std::fs::write(&output_path, &json)?;

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
        output_name: format!("{}.json", input_stem),
        output_mime: "application/json".to_string(),
        output_size,
    })
}
