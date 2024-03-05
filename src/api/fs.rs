use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shared::models::app::VulpineApp;

use super::{invoke, invoke_no_args};

pub async fn get_apps() -> Vec<String> {
    let result = invoke_no_args("get_apps").await;
    let Ok(apps) = from_value::<Vec<String>>(result) else {
        return Vec::new();
    };
    apps
}

#[derive(Serialize, Deserialize)]
struct UpdateAppArgs {
    pub name: String,
    pub app: VulpineApp,
    pub create: bool,
}

pub async fn update_app(name : String, app: VulpineApp, create: bool) -> bool {
    let result = invoke("update_app", to_value(&UpdateAppArgs {
        name,
        app,
        create,
    }).unwrap()).await;
    let Ok(apps) = from_value::<bool>(result) else {
        return false;
    };
    apps
}

#[derive(Serialize, Deserialize)]
struct GetAppArgs {
    pub name: String,
}

pub async fn get_app(name: String) -> Option<VulpineApp> {
    let result = invoke("get_app", to_value(&GetAppArgs {
        name,
    }).unwrap()).await;
    let Ok(app) = from_value::<Option<VulpineApp>>(result) else {
        return None;
    };
    app
}

pub async fn delete_app(name: String) -> bool {
    let result = invoke("delete_app", to_value(&GetAppArgs {
        name,
    }).unwrap()).await;
    let Ok(app) = from_value::<bool>(result) else {
        return false;
    };
    app
}

