use shared::models::app::AppName;

use crate::{actions::{self, get_namespace}, get_app};

#[tauri::command]
pub fn get_namespaces() -> Vec<String> {
    actions::get_namespaces()
}

#[tauri::command]
pub fn get_actions(app_handle: tauri::AppHandle, app_name: AppName, namespace: String) -> Vec<String> {
    let Some(app) = get_app(app_handle, app_name) else {
        return Vec::new();
    };
    get_namespace(&namespace).map(|ns| ns.get_actions(&app)).unwrap_or_default()
}
