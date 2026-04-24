use super::{create_temp_dir, PdfSplitOptions, ProcessError, ProcessResult};
use lopdf::Document;
use std::io::Write;

pub fn split(input_path: &str, options: &PdfSplitOptions) -> Result<ProcessResult, ProcessError> {
    let doc = Document::load(input_path)
        .map_err(|e| ProcessError::Pdf(format!("could not open pdf: {}", e)))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    let ranges = if options.pages.is_empty() {
        (1..=total_pages)
            .map(|p| (p, p))
            .collect::<Vec<(u32, u32)>>()
    } else {
        parse_page_ranges(&options.pages, total_pages)?
    };

    let temp_dir = create_temp_dir()?;
    let split_dir = temp_dir.join("split");
    std::fs::create_dir_all(&split_dir)?;

    let mut part_paths: Vec<std::path::PathBuf> = Vec::new();

    for (i, (start, end)) in ranges.iter().enumerate() {
        let mut part_doc = doc.clone();
        let pages_to_remove: Vec<u32> =
            (1..=total_pages).filter(|p| p < start || p > end).collect();

        for &page_num in pages_to_remove.iter().rev() {
            part_doc.delete_pages(&[page_num]);
        }

        let part_name = format!("part_{:02}.pdf", i + 1);
        let part_path = split_dir.join(&part_name);
        part_doc
            .save(&part_path)
            .map_err(|e| ProcessError::Pdf(format!("could not save split part: {}", e)))?;
        part_paths.push(part_path);
    }

    // Create ZIP archive
    let zip_path = temp_dir.join("split.zip");
    let zip_file = std::fs::File::create(&zip_path)?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let zip_options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for part_path in &part_paths {
        let file_name =
            part_path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or(ProcessError::Validation(
                    "could not get part filename".to_string(),
                ))?;
        zip_writer.start_file_from_path(file_name, zip_options)?;
        let bytes = std::fs::read(part_path)?;
        zip_writer.write_all(&bytes)?;
    }

    zip_writer.finish()?;

    let output_path_str = zip_path
        .to_str()
        .ok_or(ProcessError::Validation(
            "could not convert output path to string".to_string(),
        ))?
        .to_string();

    let output_size = std::fs::metadata(&zip_path).map(|m| m.len()).unwrap_or(0);

    Ok(ProcessResult {
        output_path: output_path_str,
        output_name: "split.zip".to_string(),
        output_mime: "application/zip".to_string(),
        output_size,
    })
}

fn parse_page_ranges(input: &str, total: u32) -> Result<Vec<(u32, u32)>, ProcessError> {
    let mut ranges = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() != 2 {
                return Err(ProcessError::Validation(format!(
                    "invalid page range: {}",
                    part
                )));
            }
            let start: u32 = bounds[0].trim().parse().map_err(|_| {
                ProcessError::Validation(format!("invalid page number: {}", bounds[0]))
            })?;
            let end: u32 = bounds[1].trim().parse().map_err(|_| {
                ProcessError::Validation(format!("invalid page number: {}", bounds[1]))
            })?;
            if start < 1 || end > total || start > end {
                return Err(ProcessError::Validation(format!(
                    "page range {}-{} is out of bounds (document has {} pages)",
                    start, end, total
                )));
            }
            ranges.push((start, end));
        } else {
            let page: u32 = part
                .parse()
                .map_err(|_| ProcessError::Validation(format!("invalid page number: {}", part)))?;
            if page < 1 || page > total {
                return Err(ProcessError::Validation(format!(
                    "page {} is out of bounds (document has {} pages)",
                    page, total
                )));
            }
            ranges.push((page, page));
        }
    }
    Ok(ranges)
}
