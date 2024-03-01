use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window", "getCurrent()"])]
    async fn minimize();
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window", "getCurrent()"])]
    async fn toggleMaximize();
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window", "getCurrent()"])]
    async fn close();
}

#[component]
pub fn Titlebar() -> impl IntoView {
    let on_minimize = move |_| {
        spawn_local(async move {
            minimize().await;
        });
    };
    let on_maximize = move |_| {
        spawn_local(async move {
            toggleMaximize().await;
        });
    };
    let on_close = move |_| {
        spawn_local(async move {
            close().await;
        });
    };

    view! {
        <div class="titlebar row justify-between align-center">
            <div data-tauri-drag-region class="title flex">"Vulpine"</div>
            <div class="actions row">
            <div class="titlebar-button" id="titlebar-minimize" on:click=on_minimize>
              <img
                src="https://api.iconify.design/mdi:window-minimize.svg"
                alt="minimize"
              />
            </div>
            <div class="titlebar-button" id="titlebar-maximize" on:click=on_maximize>
              <img
                src="https://api.iconify.design/mdi:window-maximize.svg"
                alt="maximize"
              />
            </div>
            <div class="titlebar-button close" id="titlebar-close" on:click=on_close>
              <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </div>
            </div>
        </div>
    }
}