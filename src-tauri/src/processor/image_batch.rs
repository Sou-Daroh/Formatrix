use super::{create_temp_dir, ImageOptions, ProcessError, ProcessResult};
use image::imageops::FilterType;
use image::GenericImageView;
use std::fs::File;
use zip::write::SimpleFileOptions;

pub fn process_batch<F>(
    input_paths: &[String],
    options: &ImageOptions,
    progress_callback: F,
) -> Result<ProcessResult, ProcessError>
where
    F: Fn(u8, &str),
{
    if input_paths.is_empty() {
        return Err(ProcessError::Validation(
            "no images provided for batch processing".to_string(),
        ));
    }

    let temp_dir = create_temp_dir()?;
    let batch_dir = temp_dir.join("batch");
    std::fs::create_dir_all(&batch_dir)?;

    let mut output_paths = Vec::new();
    let total = input_paths.len();

    for (i, input_path) in input_paths.iter().enumerate() {
        // Emit progress before starting this file
        // Max percent here is 95, reserving last 5% for zipping
        let percent = ((i as f64 / total as f64) * 95.0) as u8;
        let basename = std::path::Path::new(input_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image");
        progress_callback(
            percent,
            &format!("Processing {} ({}/{})", basename, i + 1, total),
        );

        let format_str = if options.format.is_empty() {
            super::image::detect_format(input_path)?
        } else {
            options.format.clone()
        };

        let ext = match format_str.as_str() {
            "jpeg" | "jpg" => "jpeg",
            "png" => "png",
            "webp" => "webp",
            _ => {
                return Err(ProcessError::Validation(format!(
                    "unsupported output format: {}",
                    format_str
                )))
            }
        };

        let img = image::open(input_path)?;
        let (orig_w, orig_h) = img.dimensions();
        let (new_w, new_h) =
            super::image::calculate_dimensions(orig_w, orig_h, options.width, options.height);

        let resized = if new_w != orig_w || new_h != orig_h {
            img.resize(new_w, new_h, FilterType::Lanczos3)
        } else {
            img
        };

        let input_stem = std::path::Path::new(input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
            .to_string();

        // Format name to ensure uniqueness
        let output_name = format!("{}_{:03}.{}", input_stem, i + 1, ext);
        let output_path = batch_dir.join(&output_name);

        match ext {
            "jpeg" => {
                let quality = if options.quality == 0 {
                    85
                } else {
                    options.quality
                };
                let mut writer = File::create(&output_path)?;
                let encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, quality);
                resized.write_with_encoder(encoder)?;
            }
            "png" => {
                resized.save(&output_path)?;
            }
            "webp" => {
                resized.save(&output_path)?;
            }
            _ => unreachable!(),
        }

        output_paths.push((output_name, output_path));
    }

    progress_callback(95, "Zipping batch output...");

    let zip_path = temp_dir.join("batch_output.zip");
    let zip_file = File::create(&zip_path)?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let zip_options =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (name, path) in &output_paths {
        zip_writer
            .start_file(name, zip_options)
            .map_err(|e| ProcessError::Io(e.into()))?;
        let mut f = File::open(path)?;
        std::io::copy(&mut f, &mut zip_writer)?;
    }

    zip_writer
        .finish()
        .map_err(|e| ProcessError::Io(e.into()))?;

    let output_size = std::fs::metadata(&zip_path).map(|m| m.len()).unwrap_or(0);

    let _ = std::fs::remove_dir_all(&batch_dir);

    progress_callback(100, "Done");

    Ok(ProcessResult {
        output_path: zip_path.to_string_lossy().into_owned(),
        output_name: "batch_output.zip".to_string(),
        output_mime: "application/zip".to_string(),
        output_size,
    })
}
