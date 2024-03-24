pub mod actions;
pub mod app;
pub mod details;
pub mod general;
pub mod resources;

use leptos::*;
use leptos_router::*;
use shared::models::app::VulpineApp;

use crate::{api::fs::*, components::dialog::Dialog};
use details::*;

#[component]
pub fn AppPage() -> impl IntoView {
    let params = use_params_map();
    let id =
        create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));
    view! {<HomePage app_id={id} />}
}

#[component]
pub fn HomePage(#[prop(optional, into)] app_id: MaybeProp<String>) -> impl IntoView {
    let cloned_id = app_id.clone();
    view! {
        <div class="sidebar-layout">
            <HomeListView id={app_id} />
            <HomeDetailsView id={cloned_id} />
        </div>
    }
}

#[component]
fn HomeListView(#[prop(optional_no_strip, into)] id: MaybeProp<String>) -> impl IntoView {
    let items = create_resource(|| (), |_| get_apps());
    let stored_id = store_value(id);
    let show = move || stored_id.get_value().get().is_some();
    let show_add = create_rw_signal(false);
    let show_menu = create_rw_signal(false);
    let add_name = create_rw_signal(String::new());
    let navigate = store_value(use_navigate());
    let on_app_add = move |_| {
        let name = add_name.get_untracked();
        if name.is_empty() {
            return;
        }
        spawn_local(async move {
            let result = update_app(name.to_string(), VulpineApp::default(), true).await;
            if result {
                navigate.get_value()(&format!("/apps/{}", name), Default::default());
            }
        });
    };
    view! {
        <Dialog title="Add app" show={show_add} on_close={move |_| show_add.set(false)}>
            <form method="dialog">
                <div class="form-group">
                    <label for="name">"Name"</label>
                    <input type="text" id="name" autofocus prop:value={add_name} on:input={move |ev| add_name.set(event_target_value(&ev))} />
                </div>
                <div class="row justify-end gap-xs mt-md">
                    <button class="btn secondary" on:click=on_app_add>"Cancel"</button>
                    <button class="btn primary" on:click=on_app_add>"Add"</button>
                </div>
            </form>
        </Dialog>
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
        <ul class="col min-w-md gap-xs mh-xs" class:show-sm={show}>
            <li class="row justify-between align-center ph-xs">
                <button on:click={move |_| show_menu.set(true)} class="btn secondary p-xs">
                    <img src="/public/logo.png" alt="Vulpine logo" class="big-icon" />
                </button>
                <h2 class="bold"><a href="/" class="no-decoration text">Apps</a></h2>
                <div class="row gap-xs">
                    <button on:click={move |_| show_add.set(true)} class="btn secondary row p-xs">
                        <i class="ph-light ph-plus text-icon" />
                    </button>
                </div>
            </li>
            {move || match items.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(items) => {
                    items.iter().map(|item| {
                        let cloned = item.clone();
                        let is_active = create_memo(move |_| stored_id.get_value().get().map_or(false, |id| id == *cloned));
                        view! { <a class="card no-decoration bold" class:primary=is_active class:secondary={move || !is_active.get()} href={format!("/apps/{}", &item)}><li>{item.to_string()}</li></a> }
                    }).collect_view()
                },
             }}
        </ul>
    }
}
