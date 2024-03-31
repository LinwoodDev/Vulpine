mod actions;
mod commands;
mod fs;

use commands::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet, get_apps, 
            // FS commands
            update_app, get_app, delete_app,
            // Action commands
            get_namespaces, get_actions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
