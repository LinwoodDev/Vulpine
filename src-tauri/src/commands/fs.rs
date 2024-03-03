use std::{fs, path::PathBuf};

use tauri::Manager;


#[tauri::command]
pub fn get_actions(app_handle: tauri::AppHandle) -> Vec<String> {
    let Ok(actions_dir) = get_actions_directory(app_handle) else {
        return Vec::new();
    };
    let Ok(actions) = fs::read_dir(actions_dir) else {
        return Vec::new();
    };
    // Return all files with .toml and remove .toml
    actions
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|file_name| file_name.ends_with(".toml"))
        .map(|file_name| file_name.trim_end_matches(".toml").to_string())
        .collect()
}

#[tauri::command]
pub fn create_action(app_handle: tauri::AppHandle, action_id: String) -> bool {
    let Ok(actions_dir) = get_actions_directory(app_handle) else {
        return false;
    };
    let action_file = actions_dir.join(format!("{}.toml", action_id));
    let Ok(_) = fs::write(action_file, "") else {
        return false;
    };
    true
}

pub fn get_data_directory(app_handle: tauri::AppHandle) -> tauri::Result<PathBuf> {
    app_handle.path().app_data_dir()
}

pub fn get_actions_directory(app_handle: tauri::AppHandle) -> tauri::Result<PathBuf> {
    let path = get_data_directory(app_handle).map(|data_dir| data_dir.join("Actions"));
    if let Ok(path) = &path {
        let _ = fs::create_dir_all(path);
    }
    path
}
