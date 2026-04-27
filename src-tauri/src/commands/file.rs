use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// Opens a file dialog and returns selected paths.
#[tauri::command]
pub async fn open_file_dialog(
    app: AppHandle,
    multiple: bool,
    filters: Vec<FileFilter>,
) -> Result<Vec<String>, String> {
    let result = tokio::task::spawn_blocking(move || {
        let mut builder = app.dialog().file();
        for filter in filters {
            let ext_refs: Vec<&str> = filter.extensions.iter().map(|s| s.as_str()).collect();
            builder = builder.add_filter(filter.name, &ext_refs);
        }

        if multiple {
            if let Some(paths) = builder.blocking_pick_files() {
                paths
                    .into_iter()
                    .filter_map(|p| {
                        p.into_path()
                            .ok()
                            .and_then(|p| p.to_str().map(|s| s.to_string()))
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            if let Some(path) = builder.blocking_pick_file() {
                if let Ok(p) = path.into_path() {
                    if let Some(s) = p.to_str() {
                        return vec![s.to_string()];
                    }
                }
            }
            Vec::new()
        }
    })
    .await
    .map_err(|e| format!("thread panic: {}", e))?;

    Ok(result)
}

/// Saves an output file from temp dir to user-selected location.
/// Cleans up the temp directory after successful copy.
#[tauri::command]
pub async fn save_output_file(
    app: AppHandle,
    temp_path_str: String,
    suggested_name: String,
) -> Result<String, String> {
    let selected_path = tokio::task::spawn_blocking(move || {
        app.dialog()
            .file()
            .set_file_name(&suggested_name)
            .blocking_save_file()
    })
    .await
    .map_err(|e| format!("thread panic: {}", e))?;

    if let Some(dst_path) = selected_path {
        let dst_buf: PathBuf = dst_path
            .into_path()
            .map_err(|_| "invalid path returned".to_string())?;
        std::fs::copy(&temp_path_str, &dst_buf)
            .map_err(|e| format!("failed to copy output file: {}", e))?;

        // Clean up the temp directory that held this output
        let temp_path = PathBuf::from(&temp_path_str);
        if let Some(parent) = temp_path.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }

        let out_str = dst_buf
            .to_str()
            .ok_or("invalid destination path".to_string())?
            .to_string();
        Ok(out_str)
    } else {
        // User cancelled the dialog — not an error
        Ok(String::new())
    }
}

/// Opens the system file explorer with the given file selected.
#[tauri::command]
pub fn show_in_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| format!("failed to open explorer: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| format!("failed to open Finder: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        let parent = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(path);
        std::process::Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| format!("failed to open file manager: {}", e))?;
    }
    Ok(())
}

/// Removes all stale temp directories from previous sessions.
/// Called on app startup.
#[tauri::command]
pub fn cleanup_temp_files() {
    let temp_root = std::env::temp_dir().join("formatrix");
    if temp_root.exists() {
        let _ = std::fs::remove_dir_all(&temp_root);
    }
}
