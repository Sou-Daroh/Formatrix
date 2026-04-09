use super::{create_temp_dir, ImageOptions, ProcessResult};
use image::imageops::FilterType;
use image::GenericImageView;

/// Resize and optionally re-encode an image.
///
/// Supports JPEG, PNG, WebP, GIF, BMP, and TIFF input.
/// Output formats: JPEG, PNG, WebP.
///
/// If width and height are both 0, the original dimensions are preserved.
/// If only one dimension is set, aspect ratio is maintained.
/// If both are set, the image is fit within the bounding box (no crop).
pub fn process(input_path: &str, options: &ImageOptions) -> Result<ProcessResult, String> {
    let img = image::open(input_path).map_err(|e| format!("could not open image: {}", e))?;

    let (orig_w, orig_h) = img.dimensions();
    let (new_w, new_h) = calculate_dimensions(orig_w, orig_h, options.width, options.height);

    let resized = if new_w != orig_w || new_h != orig_h {
        img.resize(new_w, new_h, FilterType::Lanczos3)
    } else {
        img
    };

    let format_str = if options.format.is_empty() {
        detect_format(input_path)?
    } else {
        options.format.clone()
    };

    let ext = match format_str.as_str() {
        "jpeg" | "jpg" => "jpeg",
        "png" => "png",
        "webp" => "webp",
        _ => return Err(format!("unsupported output format: {}", format_str)),
    };

    let temp_dir = create_temp_dir()?;
    let output_path = temp_dir.join(format!("output.{}", ext));

    match ext {
        "jpeg" => {
            let quality = if options.quality == 0 {
                85
            } else {
                options.quality
            };
            let mut writer = std::fs::File::create(&output_path)
                .map_err(|e| format!("could not create output file: {}", e))?;
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, quality);
            resized
                .write_with_encoder(encoder)
                .map_err(|e| format!("could not encode jpeg: {}", e))?;
        }
        "png" => {
            resized
                .save(&output_path)
                .map_err(|e| format!("could not save png: {}", e))?;
        }
        "webp" => {
            resized
                .save(&output_path)
                .map_err(|e| format!("could not save webp: {}", e))?;
        }
        _ => return Err(format!("unsupported output format: {}", ext)),
    }

    let output_path_str = output_path
        .to_str()
        .ok_or("could not convert output path to string")?
        .to_string();

    // Derive output name from input filename
    let input_stem = std::path::Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let output_size = std::fs::metadata(&output_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(ProcessResult {
        output_path: output_path_str,
        output_name: format!("{}.{}", input_stem, ext),
        output_mime: format!("image/{}", ext),
        output_size,
    })
}

/// Calculate output dimensions preserving aspect ratio.
///
/// - (0, 0) -> original dimensions
/// - (w, 0) -> scale to width, maintain ratio
/// - (0, h) -> scale to height, maintain ratio
/// - (w, h) -> fit within bounding box, no crop
fn calculate_dimensions(orig_w: u32, orig_h: u32, target_w: u32, target_h: u32) -> (u32, u32) {
    match (target_w, target_h) {
        (0, 0) => (orig_w, orig_h),
        (w, 0) => {
            let ratio = w as f64 / orig_w as f64;
            (w, (orig_h as f64 * ratio).round() as u32)
        }
        (0, h) => {
            let ratio = h as f64 / orig_h as f64;
            ((orig_w as f64 * ratio).round() as u32, h)
        }
        (w, h) => {
            let ratio_w = w as f64 / orig_w as f64;
            let ratio_h = h as f64 / orig_h as f64;
            let ratio = ratio_w.min(ratio_h);
            (
                (orig_w as f64 * ratio).round() as u32,
                (orig_h as f64 * ratio).round() as u32,
            )
        }
    }
}

/// Detect output format from the input file extension.
fn detect_format(path: &str) -> Result<String, String> {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => Ok("jpeg".to_string()),
        "png" => Ok("png".to_string()),
        "webp" => Ok("webp".to_string()),
        "gif" => Ok("jpeg".to_string()),
        "bmp" => Ok("png".to_string()),
        "tiff" | "tif" => Ok("png".to_string()),
        _ => Err(format!("could not detect format from extension: {}", ext)),
    }
}
