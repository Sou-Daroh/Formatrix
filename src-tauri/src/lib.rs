pub mod commands;
pub mod processor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_next::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::process::process_image,
            commands::process::process_image_batch,
            commands::process::process_csv_json,
            commands::process::process_pdf_text,
            commands::process::process_pdf_merge,
            commands::process::process_pdf_split,
            commands::file::open_file_dialog,
            commands::file::save_output_file,
            commands::file::cleanup_temp_files,
        ])
        .setup(|_app| {
            // Clean stale temp files from previous sessions on startup
            let temp_root = std::env::temp_dir().join("formatrix");
            if temp_root.exists() {
                let _ = std::fs::remove_dir_all(&temp_root);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
