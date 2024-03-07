use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shared::models::app::VulpineApp;

use super::{invoke, invoke_no_args};

#[derive(Serialize, Deserialize)]
struct Output<T> {
    pub id: i8,
    pub message: T,
}

pub async fn get_apps() -> Vec<String> {
    let result = invoke_no_args("get_apps").await;
    let output = from_value::<Output<Vec<String>>>(result).unwrap();
    output.message
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
    from_value::<bool>(result).unwrap()
}

#[derive(Serialize, Deserialize)]
struct GetAppArgs {
    pub name: String,
}

pub async fn get_app(name: String) -> Option<VulpineApp> {
    let result = invoke("get_app", to_value(&GetAppArgs {
        name,
    }).unwrap()).await;
    let output = from_value::<Output<Option<VulpineApp>>>(result).unwrap();
    output.message
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

