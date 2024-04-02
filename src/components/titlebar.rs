use leptos::*;
use wasm_bindgen::prelude::*;
use crate::components::dialog::Dialog;

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
    let show_menu = create_rw_signal(false);
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
      <Dialog title="Vulpine" show={show_menu} on_close={move |_| show_menu.set(false)}>
          <div class="col w-lg max-w-full gap-xs mh-sm">
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-gear text-icon" />
                  "Import"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-gear text-icon" />
                  "Export"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-gear text-icon" />
                  "Settings"
              </a>
              <hr />
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-article text-icon" />
                  "Documentation"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-flag text-icon" />
                  "Release notes"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-users text-icon" />
                  "Matrix"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-users text-icon" />
                  "Discord"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-translate text-icon" />
                  "Crowdin"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-code text-icon" />
                  "Source"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-arrow-counter-clockwise text-icon" />
                  "Changelog"
              </a>
              <hr />
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-stack text-icon" />
                  "License"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-identification-card text-icon" />
                  "Imprint"
              </a>
              <a href="/settings" class="btn card secondary row bold">
                  <i class="ph-light ph-shield text-icon" />
                  "Privacy policy"
              </a>
          </div>
      </Dialog>
      <div class="titlebar card paper row justify-between align-center mp-none">
          <button on:click={move |_| show_menu.set(true)} class="btn secondary p-xs m-xs">
              <img src="/public/logo.png" alt="Vulpine logo" class="big-icon" />
          </button>
          <div data-tauri-drag-region class="title flex">"Vulpine"</div>
          <div class="actions row align-self-start gap-xs">
            <div class="btn secondary p-xs" id="titlebar-minimize" on:click=on_minimize>
              <i class="ph-light ph-minus text-icon" />
            </div>
            <div class="btn secondary p-xs" id="titlebar-maximize" on:click=on_maximize>
              <i class="ph-light ph-square text-icon" />
            </div>
            <div class="btn secondary close p-xs" id="titlebar-close" on:click=on_close>
              <i class="ph-light ph-x text-icon" />
            </div>
          </div>
      </div>
    }
}
