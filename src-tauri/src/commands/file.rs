use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

/// Opens a file dialog and returns selected paths.
#[tauri::command]
pub async fn open_file_dialog(
    app: AppHandle,
    multiple: bool,
    filters: Vec<(&str, Vec<&str>)>,
) -> Result<Vec<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut dialog = app.dialog().file();
    for (name, extensions) in filters {
        dialog = dialog.add_filter(name, &extensions);
    }

    if multiple {
        dialog.pick_files(move |file_paths| {
            let paths = match file_paths {
                Some(paths) => paths
                    .into_iter()
                    .filter_map(|p| {
                        p.into_path()
                            .ok()
                            .and_then(|p| p.to_str().map(|s| s.to_string()))
                    })
                    .collect::<Vec<String>>(),
                None => Vec::new(),
            };
            tx.send(paths).unwrap();
        });
    } else {
        dialog.pick_file(move |file_path| {
            let paths = match file_path {
                Some(path) => {
                    if let Ok(p) = path.into_path() {
                        if let Some(s) = p.to_str() {
                            vec![s.to_string()]
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                }
                None => Vec::new(),
            };
            tx.send(paths).unwrap();
        });
    }

    let result = tokio::task::spawn_blocking(move || rx.recv().unwrap_or_default())
        .await
        .map_err(|e| format!("thread panic: {}", e))?;

    Ok(result)
}

/// Saves an output file from temp dir to user-selected location.
#[tauri::command]
pub async fn save_output_file(
    app: AppHandle,
    temp_path_str: String,
    suggested_name: String,
) -> Result<String, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    app.dialog()
        .file()
        .set_file_name(&suggested_name)
        .save_file(move |file_path| {
            tx.send(file_path).unwrap();
        });

    let selected_path = tokio::task::spawn_blocking(move || rx.recv().unwrap_or_default())
        .await
        .map_err(|e| format!("thread panic: {}", e))?;

    if let Some(dst_path) = selected_path {
        let dst_buf: PathBuf = dst_path
            .into_path()
            .map_err(|_| "invalid path returned".to_string())?;
        std::fs::copy(&temp_path_str, &dst_buf)
            .map_err(|e| format!("failed to copy output file: {}", e))?;

        let out_str = dst_buf
            .to_str()
            .ok_or("invalid destination path".to_string())?
            .to_string();
        Ok(out_str)
    } else {
        Err("user cancelled".to_string())
    }
}
