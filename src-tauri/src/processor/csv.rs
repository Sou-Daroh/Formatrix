use super::{create_temp_dir, CsvOptions, ProcessResult};

/// Convert a CSV file to JSON.
///
/// Each row becomes a JSON object with keys from the CSV header.
/// All values are strings -- no type inference is performed.
/// Missing fields (rows shorter than the header) become empty strings.
pub fn process(input_path: &str, options: &CsvOptions) -> Result<ProcessResult, String> {
    let mut reader = csv::Reader::from_path(input_path)
        .map_err(|e| format!("could not open csv file: {}", e))?;

    let headers = reader
        .headers()
        .map_err(|e| format!("could not read csv headers: {}", e))?
        .iter()
        .map(|h| h.trim().to_string())
        .collect::<Vec<String>>();

    let mut records: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();

    for result in reader.records() {
        let record = result.map_err(|e| format!("could not read csv row: {}", e))?;
        let mut map = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            let value = record.get(i).unwrap_or("").to_string();
            map.insert(header.clone(), serde_json::Value::String(value));
        }
        records.push(map);
    }

    let json = if options.pretty {
        serde_json::to_string_pretty(&records)
    } else {
        serde_json::to_string(&records)
    }
    .map_err(|e| format!("could not serialise json: {}", e))?;

    let temp_dir = create_temp_dir()?;
    let output_path = temp_dir.join("output.json");
    std::fs::write(&output_path, &json)
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
        output_name: format!("{}.json", input_stem),
        output_mime: "application/json".to_string(),
        output_size,
    })
}
