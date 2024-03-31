use std::{fs, path::PathBuf};

use shared::models::app::AppName;
use tauri::Manager;


pub fn get_data_directory(app_handle: &tauri::AppHandle) -> tauri::Result<PathBuf> {
    app_handle
        .path()
        .document_dir()
        .map(|path| path.join("Linwood/Vulpine"))
}

pub fn get_apps_directory(app_handle: &tauri::AppHandle) -> tauri::Result<PathBuf> {
    let path = get_data_directory(app_handle).map(|data_dir| data_dir.join("Apps"));
    if let Ok(path) = &path {
        let _ = fs::create_dir_all(path);
    }
    path
}

pub fn get_app_file(app_handle: &tauri::AppHandle, name: &AppName) -> Option<PathBuf> {
    let name = name.as_filename();
    let Ok(apps_dir) = get_apps_directory(app_handle) else {
        return None;
    };
    Some(apps_dir.join(name))
}
