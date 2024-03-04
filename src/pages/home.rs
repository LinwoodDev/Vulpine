use leptos::{logging::log, *};
use leptos_router::*;
use shared::models::app::VulpineApp;

use crate::utils::signal::create_initial_rw_signal;

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
    const ITEMS: [&str; 3] = ["Item 1", "Item 2", "Item 3"];
    let stored_id = store_value(id);
    let show = move || stored_id.get_value().get().is_some();
    view! {
        <ul class="col min-w-md gap-xs mh-xs" class:show-sm={show}>
            <li class="row justify-between align-center ph-xs">
                <h2 class="bold"><a href="/" class="no-decoration normal-color">Apps</a></h2>
                <div class="row gap-xs">
                    <a href="/apps" class="btn secondary p-xs"><img class="invert icon" title="Add" src="public/icons/plus-light.svg" alt="Plus icon"/></a>
                    <a href="/settings" class="btn secondary p-xs"><img class="invert icon" title="Settings" src="public/icons/gear-light.svg" alt="Gear icon"/></a>
                </div>
            </li>
            { ITEMS.iter().map(move |item| {
                let is_active = move || stored_id.get_value().get().map_or(false, |id| id == *item);
                view! { <a class="card primary no-decoration bold" class:paper=is_active href={format!("/apps/{}", item)}><li>{item.to_string()}</li></a> }
            }).collect_view() }
        </ul>
    }
}

#[component]
fn HomeDetailsView(#[prop(optional_no_strip, into)] id: MaybeProp<String>) -> impl IntoView {
    let stored_id = store_value(id);
    let show = move || stored_id.get_value().get().is_none();
    let title = move || {
        let id = stored_id.get_value().get().unwrap_or_default();
        if id.is_empty() {
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
            log!("fetching app: {}", id);
            //if id.is_empty() {
            return Some(VulpineApp::default());
            //};
            //let args = to_value(&id).unwrap();
            //let response = invoke("get_app", args).await;
            //let Ok(app) = from_value(response) else {
            //    return None;
            //};
            //Some(app)
        },
    );
    let initial_current_id = create_memo(move |_| stored_id.get_value().get().unwrap_or_default());
    let current_id = create_initial_rw_signal(initial_current_id);
    let app = create_rw_signal(VulpineApp::default());
    create_effect(move |_| {
        app.set(fetched.get().flatten().unwrap_or_default());
    });
    view! {
        <div class="col card paper flex ph-sm min-w-md h-full overflow-y" class:show-sm={move || stored_id.get_value().get().is_none()}>
        <Show when={move || !show()}>
            <div class="row align-center gap-sm">
                <a href="/" class="btn secondary p-xs hide-sm"><img class="invert icon" title="Home" src="public/icons/house-light.svg" alt="House icon"/></a>
                <h2 class="bold">
                    {title()}
                </h2>
                <input type="text" class="flex" prop:value={move || current_id.get()} on:input={move |ev| current_id.set(event_target_value(&ev))} />
                <button class="btn primary">"Save"</button>
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
