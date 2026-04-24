use crate::processor::{
    self, CsvOptions, ImageOptions, PdfSplitOptions, ProcessError, ProcessResult, ProgressPayload,
};
use std::time::Duration;
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

/// Runs a CPU-bound task in a blocking thread while smoothly emitting asymptotic progress updates.
/// Converts ProcessError to String at the IPC boundary so the frontend receives readable messages.
async fn run_with_progress<F>(
    app: &AppHandle,
    stage_name: &str,
    task: F,
) -> Result<ProcessResult, String>
where
    F: FnOnce() -> Result<ProcessResult, ProcessError> + Send + 'static,
{
    let app_clone = app.clone();
    let stage_clone = stage_name.to_string();

    emit_progress(&app_clone, 5, &stage_clone);

    // Asymptotic progress loop: ticks up quickly at first, then slows down, approaching 95%
    let progress_task = tokio::spawn(async move {
        let mut percent: u8 = 10;
        let mut delay_ms: u64 = 100;
        loop {
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            percent += (100 - percent) / 8;
            if percent > 95 {
                percent = 95;
            }
            emit_progress(&app_clone, percent, &stage_clone);
            delay_ms = (delay_ms as f64 * 1.2) as u64;
        }
    });

    // Run actual CPU-bound task
    let result = tokio::task::spawn_blocking(task)
        .await
        .map_err(|e| format!("thread panic: {}", e))?;

    // Task finished! Kill the progress loop
    progress_task.abort();

    // On error, clean up any orphaned temp files created during processing
    if let Err(ref _e) = result {
        let temp_root = std::env::temp_dir().join("formatrix");
        if temp_root.exists() {
            let _ = std::fs::remove_dir_all(&temp_root);
        }
    }

    // UI snaps to 100% on success in App.svelte automatically, but we can emit a 100 just in case
    emit_progress(app, 100, "Finalizing...");

    // Convert ProcessError → String at the IPC boundary
    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_image(
    app: AppHandle,
    input_path: String,
    options: ImageOptions,
) -> Result<ProcessResult, String> {
    run_with_progress(&app, "Resizing image...", move || {
        processor::image::process(&input_path, &options)
    })
    .await
}

#[tauri::command]
pub async fn process_csv_json(
    app: AppHandle,
    input_path: String,
    options: CsvOptions,
) -> Result<ProcessResult, String> {
    run_with_progress(&app, "Converting CSV...", move || {
        processor::csv::process(&input_path, &options)
    })
    .await
}

#[tauri::command]
pub async fn process_pdf_text(app: AppHandle, input_path: String) -> Result<ProcessResult, String> {
    run_with_progress(&app, "Extracting text...", move || {
        processor::pdf_text::process(&input_path)
    })
    .await
}

#[tauri::command]
pub async fn process_pdf_merge(
    app: AppHandle,
    input_paths: Vec<String>,
) -> Result<ProcessResult, String> {
    run_with_progress(&app, "Merging PDFs...", move || {
        processor::pdf_merge::merge(&input_paths)
    })
    .await
}

#[tauri::command]
pub async fn process_pdf_split(
    app: AppHandle,
    input_path: String,
    options: PdfSplitOptions,
) -> Result<ProcessResult, String> {
    run_with_progress(&app, "Splitting PDF...", move || {
        processor::pdf_split::split(&input_path, &options)
    })
    .await
}
