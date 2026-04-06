pub mod commands;
pub mod processor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::process::process_image,
            commands::process::process_csv_json,
            commands::process::process_pdf_text,
            commands::process::process_pdf_merge,
            commands::process::process_pdf_split,
            commands::file::open_file_dialog,
            commands::file::save_output_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
