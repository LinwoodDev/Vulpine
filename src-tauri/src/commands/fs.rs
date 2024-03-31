use std::fs;

use shared::models::app::{AppName, VulpineApp};

use crate::fs::{get_app_file, get_apps_directory};

#[tauri::command]
pub fn get_apps(app_handle: tauri::AppHandle) -> Vec<String> {
    let Ok(actions_dir) = get_apps_directory(&app_handle) else {
        return Vec::new();
    };
    let Ok(app_dir) = fs::read_dir(actions_dir) else {
        return Vec::new();
    };
    // Return all folders that include a app.toml file
    let found = app_dir
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().map(|ext| ext == "toml").unwrap_or(false))
        .map(|path| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
                .unwrap_or_default()
        })
        .collect();
    found
}

#[tauri::command]
pub fn get_app(app_handle: tauri::AppHandle, name: AppName) -> Option<VulpineApp> {
    let Some(app_file) = get_app_file(&app_handle, &name) else {
        return None;
    };
    let Ok(app_toml) = fs::read_to_string(app_file) else {
        return None;
    };
    let Ok(app) = toml::from_str(&app_toml) else {
        return None;
    };
    Some(app)
}

#[tauri::command]
pub fn update_app(
    app_handle: tauri::AppHandle,
    name: AppName,
    app: VulpineApp,
    create: bool,
) -> bool {
    let Ok(apps_dir) = get_apps_directory(&app_handle) else {
        return false;
    };
    let name = name.as_filename();
    println!("Updating app: {:?}", app);
    let app_file = apps_dir.join(name);
    let Ok(app_toml) = toml::to_string_pretty(&app) else {
        return false;
    };
    if app_file.exists() && create {
        return false;
    }
    if fs::write(app_file, app_toml).is_err() {
        return false;
    };
    true
}

#[tauri::command]
pub fn delete_app(app_handle: tauri::AppHandle, name: AppName) -> bool {
    let Some(app_file) = get_app_file(&app_handle, &name) else {
        return false;
    };
    let Ok(_) = fs::remove_file(app_file) else {
        return false;
    };
    true
}
