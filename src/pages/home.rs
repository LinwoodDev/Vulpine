use leptos::{logging::log, *};
use leptos_router::*;
use shared::models::app::VulpineApp;

use crate::{api::fs::*, utils::signal::create_initial_rw_signal};

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
    view! {
        <ul class="col min-w-md gap-xs mh-xs" class:show-sm={show}>
            <li class="row justify-between align-center ph-xs">
                <h2 class="bold"><a href="/" class="no-decoration normal-color">Apps</a></h2>
                <div class="row gap-xs">
                    <a href="/apps" class="btn secondary p-xs"><img class="invert icon" src="/public/icons/plus-light.svg" alt="Plus icon"/></a>
                    <a href="/settings" class="btn secondary p-xs"><img class="invert icon" title="Settings" src="/public/icons/gear-light.svg" alt="Gear icon"/></a>
                </div>
            </li>
            {move || match items.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(items) => {
                    items.iter().map(|item| {
                        let cloned = item.clone();
                        let is_active = move || stored_id.get_value().get().map_or(false, |id| id == *cloned);
                        view! { <a class="card primary no-decoration bold" class:paper=is_active href={format!("/apps/{}", &item)}><li>{item.to_string()}</li></a> }
                    }).collect_view()
                },
             }}
        </ul>
    }
}

#[component]
fn HomeDetailsView(#[prop(optional_no_strip, into)] id: MaybeProp<String>) -> impl IntoView {
    let navigate = store_value(use_navigate());
    let stored_id = store_value(id);
    let show = move || stored_id.get_value().get().is_none();
    let is_adding = move || stored_id.get_value().get().unwrap_or_default().is_empty();
    let title = move || {
        if is_adding() {
            "Add".to_string()
        } else {
            "Edit".to_string()
        }
    };
    let fetched = create_local_resource(
        move || stored_id.get_value().get(),
        |id| async {
            let Some(id) = id else {
                return None;
            };
            get_app(id).await
        },
    );
    let initial_current_id = create_memo(move |_| stored_id.get_value().get().unwrap_or_default());
    let current_id = create_initial_rw_signal(initial_current_id);
    let app = create_rw_signal(VulpineApp::default());
    create_effect(move |_| {
        app.set(fetched.get().flatten().unwrap_or_default());
    });
    let on_save = move |_| {
        spawn_local(async move {
            let current_id = current_id.get_untracked();
            let app = app.get_untracked();
            if update_app(current_id, app.clone(), stored_id.get_value().get_untracked().unwrap_or_default().is_empty()).await {
                fetched.try_update(|_| Some(app));
            }
        });
    };
    let on_delete = move |_| {
        spawn_local(async move {
            let id = current_id.get_untracked();
            if delete_app(id).await {
                navigate.get_value()("", Default::default());
            }
        });
    };
    view! {
        <div class="col card paper flex ph-sm min-w-md h-full overflow-y" class:show-sm={move || stored_id.get_value().get().is_none()}>
        <Show when={move || !show()}>
            <div class="row align-center gap-sm">
                <a href="/" class="btn secondary p-xs hide-sm"><img class="invert icon" title="Home" src="/public/icons/house-light.svg" alt="House icon"/></a>
                <h2 class="bold">
                    {title()}
                </h2>
                <input type="text" class="flex" readonly={move || !is_adding()} prop:value={move || current_id.get()} on:input={move |ev| current_id.set(event_target_value(&ev))} />
                <button class="btn primary" disabled={move || fetched.get().flatten().map_or(false, |f| app.get() == f)} on:click=on_save>"Save"</button>
                <Show when={move || !is_adding()}>
                    <button class="btn secondary" on:click=on_delete><img class="invert icon" title="Delete" src="/public/icons/trash-light.svg" alt="Trash icon"/></button>
                </Show>
            </div>
        </Show>
        <div class="flex col justify-center">
        {move || match stored_id.get_value().get() {
            Some(_) => view!{
                <AppDetailsView app={app} />
            }.into_view(),
            None => view! {<p class="text-center">"Nothing selected"</p>}.into_view()
        }}
        </div>
        </div>
    }
}

#[component]
pub fn AppDetailsView(app: RwSignal<VulpineApp>) -> impl IntoView {
    const TABS: [&str; 3] = ["Tab 1", "Tab 2", "Tab 3"];
    log!("AppDetailsView");
    view! {<div class="col gap-xs flex h-full">
        <div class="form-group">
            <label for="name">"Name"</label>
            <input type="text" id="name"
                prop:value={move || app.get().name}
                on:input={move |ev| {
                    let name = event_target_value(&ev);
                    app.update(|app| app.name = name);
                }} />
        </div>
        <div class="form-group">
            <label for="version">"Version"</label>
            <input type="text" id="version"
                prop:value={move || app.get().version}
                on:input={move |ev| {
                    let version = event_target_value(&ev);
                    app.update(|app| app.version = version);
                }} />
        </div>
        <div class="form-group">
            <label for="description">"Description"</label>
            <textarea id="description"
                class="resize-vertical h-xs max-h-xl min-h-xs"
                prop:value={move || app.get().description}
                on:input={move |ev| {
                    let description = event_target_value(&ev);
                    app.update(|app| app.description = description);
                }} />
        </div>
        <h3>"Actions"</h3>
        <div class="overflow-x row gap-xs align-center">
        { TABS.iter().map(|tab| view! {<a href="#" class="btn p-xs">{tab.to_string()}</a>}).collect_view() }
        <a href="#" class="btn p-xs"><img class="invert icon" title="Add" src="public/icons/plus-light.svg" alt="Plus icon"/></a>
        </div>
        Value: {move || format!("{:?}", app.get())}
    </div>}
}
