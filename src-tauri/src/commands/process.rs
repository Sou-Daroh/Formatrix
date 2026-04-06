use crate::processor::{
    self, CsvOptions, ImageOptions, PdfSplitOptions, ProcessResult, ProgressPayload,
};
use tauri::{AppHandle, Emitter};

// Helper to emit progress events
fn emit_progress(app: &AppHandle, percent: u8, stage: &str) {
    let _ = app.emit(
        "progress",
        ProgressPayload {
            percent,
            stage: stage.to_string(),
        },
    );
}

#[tauri::command]
pub async fn process_image(
    app: AppHandle,
    input_path: String,
    options: ImageOptions,
) -> Result<ProcessResult, String> {
    emit_progress(&app, 10, "Starting image processing...");

    // CPU-bound task runs in a separate thread pool
    tokio::task::spawn_blocking(move || {
        let result = processor::image::process(&input_path, &options);
        // We can't emit progress _from within_ the sync blocking task easily without a channel,
        // but for short tasks this is fine.
        result
    })
    .await
    .map_err(|e| format!("thread panic: {}", e))?
}

#[tauri::command]
pub async fn process_csv_json(
    app: AppHandle,
    input_path: String,
    options: CsvOptions,
) -> Result<ProcessResult, String> {
    emit_progress(&app, 10, "Starting CSV conversion...");

    tokio::task::spawn_blocking(move || processor::csv::process(&input_path, &options))
        .await
        .map_err(|e| format!("thread panic: {}", e))?
}

#[tauri::command]
pub async fn process_pdf_text(app: AppHandle, input_path: String) -> Result<ProcessResult, String> {
    emit_progress(&app, 10, "Extracting PDF text...");

    tokio::task::spawn_blocking(move || processor::pdf_text::process(&input_path))
        .await
        .map_err(|e| format!("thread panic: {}", e))?
}

#[tauri::command]
pub async fn process_pdf_merge(
    app: AppHandle,
    input_paths: Vec<String>,
) -> Result<ProcessResult, String> {
    emit_progress(&app, 10, "Merging PDFs...");

    tokio::task::spawn_blocking(move || processor::pdf_merge::merge(&input_paths))
        .await
        .map_err(|e| format!("thread panic: {}", e))?
}

#[tauri::command]
pub async fn process_pdf_split(
    app: AppHandle,
    input_path: String,
    options: PdfSplitOptions,
) -> Result<ProcessResult, String> {
    emit_progress(&app, 10, "Splitting PDF...");

    tokio::task::spawn_blocking(move || processor::pdf_merge::split(&input_path, &options))
        .await
        .map_err(|e| format!("thread panic: {}", e))?
}
