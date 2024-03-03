use std::{fs, path::PathBuf};

use shared::models::app::VulpineApp;
use tauri::Manager;


#[tauri::command]
pub fn get_apps(app_handle: tauri::AppHandle) -> Vec<String> {
    let Ok(actions_dir) = get_apps_directory(&app_handle) else {
        return Vec::new();
    };
    let Ok(app) = fs::read_dir(actions_dir) else {
        return Vec::new();
    };
    // Return all files with .toml and remove .toml
    app
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|file_name| file_name.ends_with(".toml"))
        .map(|file_name| file_name.trim_end_matches(".toml").to_string())
        .collect()
}

#[tauri::command]
pub fn create_app(app_handle: tauri::AppHandle, name: String) -> Option<VulpineApp> {
    let Ok(apps_dir) = get_apps_directory(&app_handle) else {
        return None;
    };
    let file_name = safe_filename(&name);
    let app = VulpineApp {
        name: name.clone(),
        ..Default::default()
    };
    let app_dir = apps_dir.join(file_name);
    if app_dir.exists() {
        return None;
    }
    let Ok(_) = fs::create_dir(&app_dir) else {
        return None;
    };
    if update_app(app_handle, name, app.clone()) {
        Some(app)
    } else {
        fs::remove_dir_all(app_dir).ok();
        None
    }
}

#[tauri::command]
pub fn update_app(app_handle: tauri::AppHandle, name: String, app: VulpineApp) -> bool {
    let Ok(apps_dir) = get_apps_directory(&app_handle) else {
        return false;
    };
    let file_name = safe_filename(&name);
    let app_dir = apps_dir.join(file_name);
    let app_file = app_dir.join("app.toml");
    let Ok(app_toml) = toml::to_string_pretty(&app) else {
        return false;
    };
    if app_file.exists() {
        return false;
    }
    let Ok(_) = fs::write(app_file, app_toml) else {
        return false;
    };
    true
}

const ALLOWED_SPECIAL_CHARS: &str = "_- ";

pub fn safe_filename(name: &str) -> String {
    name.replace(|c: char| !c.is_ascii_alphanumeric() || !ALLOWED_SPECIAL_CHARS.contains(c), "")
}

pub fn get_data_directory(app_handle: &tauri::AppHandle) -> tauri::Result<PathBuf> {
    app_handle.path().app_data_dir()
}

pub fn get_apps_directory(app_handle: &tauri::AppHandle) -> tauri::Result<PathBuf> {
    let path = get_data_directory(app_handle).map(|data_dir| data_dir.join("Apps"));
    if let Ok(path) = &path {
        let _ = fs::create_dir_all(path);
    }
    path
}
