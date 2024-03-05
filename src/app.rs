use crate::{api::invoke, layouts::main::MainLayout, pages::{home::*, settings::*}};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
        <MainLayout>
        <Routes>
            <Route path="" view={|| view! {<HomePage />}} />
            <Route path="apps" view={|| view! {<HomePage app_id="".to_string() />}} />
            <Route path="apps/:id" view=AppPage />
            <Route path="settings" view=SettingsPage />
            <Route path="test" view=TestPage />
        </Routes>
        </MainLayout>
        </Router>
    }
}

#[component]
pub fn TestPage() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <a href="/" class="btn card primary">"Go back"</a>
        <div class="row">
            <a href="https://tauri.app" target="_blank">
                <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            </a>
            <a href="https://docs.rs/leptos/" target="_blank">
                <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
            </a>
        </div>
        <p>"Click on the Tauri and Leptos logos to learn more."</p>
        <p>
            "Recommended IDE setup: "
            <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
            " + "
            <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
            " + "
            <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
        </p>
        <form class="row" on:submit=greet>
            <input
                id="greet-input"
                placeholder="Enter a name..."
                on:input=update_name
            />
            <button type="submit">"Greet"</button>
        </form>
        <p><b>{ move || greet_msg.get() }</b></p>
    }
}
