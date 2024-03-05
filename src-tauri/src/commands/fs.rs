use std::{fs, path::PathBuf};

use shared::models::app::VulpineApp;
use tauri::Manager;


#[tauri::command]
pub fn get_apps(app_handle: tauri::AppHandle) -> Vec<String> {
    let Ok(actions_dir) = get_apps_directory(&app_handle) else {
        return Vec::new();
    };
    let Ok(app_dir) = fs::read_dir(actions_dir) else {
        return Vec::new();
    };
    // Return all folders that include a app.toml file
    app_dir
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter(|path| path.join("app.toml").exists())
        .filter_map(|path| path.file_name().map(|name| name.to_string_lossy().to_string()))
        .collect()
}

#[tauri::command]
pub fn get_app(app_handle: tauri::AppHandle, name: String) -> Option<VulpineApp> {
    let Ok(apps_dir) = get_apps_directory(&app_handle) else {
        return None;
    };
    let file_name = safe_filename(&name);
    if file_name.is_empty() {
        return None;
    }
    let app_dir = apps_dir.join(file_name);
    let app_file = app_dir.join("app.toml");
    if !app_file.exists() {
        return None;
    }
    let Ok(app_toml) = fs::read_to_string(app_file) else {
        return None;
    };
    let Ok(app) = toml::from_str(&app_toml) else {
        return None;
    };
    Some(app)
}

#[tauri::command]
pub fn update_app(app_handle: tauri::AppHandle, name: String, app: VulpineApp, create : bool) -> bool {
    let Ok(apps_dir) = get_apps_directory(&app_handle) else {
        return false;
    };
    let file_name = safe_filename(&name);
    if file_name.is_empty() {
        return false;
    }
    let app_dir = apps_dir.join(file_name);
    let app_file = app_dir.join("app.toml");
    let Ok(app_toml) = toml::to_string_pretty(&app) else {
        return false;
    };
    if app_file.exists() && create {
        return false;
    }
    if fs::create_dir_all(app_dir).is_err() {
        return false;
    }
    let Ok(_) = fs::write(app_file, app_toml) else {
        return false;
    };
    true
}

const ALLOWED_SPECIAL_CHARS: &str = "_- ";

pub fn safe_filename(name: &str) -> String {
    name.trim().replace(|c: char| !c.is_ascii_alphanumeric() && !ALLOWED_SPECIAL_CHARS.contains(c), "")
}

pub fn get_data_directory(app_handle: &tauri::AppHandle) -> tauri::Result<PathBuf> {
    app_handle.path().document_dir().map(|path| path.join("Linwood/Vulpine"))
}

pub fn get_apps_directory(app_handle: &tauri::AppHandle) -> tauri::Result<PathBuf> {
    let path = get_data_directory(app_handle).map(|data_dir| data_dir.join("Apps"));
    if let Ok(path) = &path {
        let _ = fs::create_dir_all(path);
    }
    path
}
