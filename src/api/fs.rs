use leptos::logging::error;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shared::models::app::{AppName, VulpineApp};

use super::{invoke, invoke_no_args};

#[derive(Serialize, Deserialize)]
struct Output<T> {
    pub id: i8,
    pub message: T,
}

pub async fn get_apps() -> Vec<String> {
    let result = invoke_no_args("get_apps").await;
    let Ok(output) = from_value::<Vec<String>>(result.clone()) else {
        error!("Failed to get apps. Result: {:?}", result);
        return Vec::new();
    };
    output
}

#[derive(Serialize, Deserialize)]
struct UpdateAppArgs {
    pub name: AppName,
    pub app: VulpineApp,
    pub create: bool,
}

pub async fn update_app(name: String, app: VulpineApp, create: bool) -> bool {
    let Some(name) = AppName::parse(&name) else {
        return false;
    };
    let result = invoke(
        "update_app",
        to_value(&UpdateAppArgs { name, app, create }).unwrap(),
    )
    .await;
    let Ok(result) = from_value::<bool>(result.clone()) else {
        error!("Failed to update app {:?}", result);
        return false;
    };
    result
}

#[derive(Serialize, Deserialize)]
struct GetAppArgs {
    pub name: AppName,
}

pub async fn get_app(name: String) -> Option<VulpineApp> {
    let Some(name) = AppName::parse(&name) else {
        return None;
    };
    let result = invoke("get_app", to_value(&GetAppArgs { name }).unwrap()).await;
    let Ok(output) = from_value::<Option<VulpineApp>>(result.clone()) else {
        error!("Failed to get app {:?}", result);
        return None;
    };
    output
}

pub async fn delete_app(name: String) -> bool {
    let Some(name) = AppName::parse(&name) else {
        return false;
    };
    let args = to_value(&GetAppArgs { name }).unwrap();
    let result = invoke("delete_app", args).await;
    let Ok(app) = from_value::<bool>(result) else {
        return false;
    };
    app
}
