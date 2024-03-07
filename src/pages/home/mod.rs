pub mod app;
pub mod details;

use leptos::*;
use leptos_router::*;

use crate::api::fs::*;
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
                        let is_active = create_memo(move |_| stored_id.get_value().get().map_or(false, |id| id == *cloned));
                        view! { <a class="card no-decoration bold" class:primary=is_active class:secondary={move || !is_active.get()} href={format!("/apps/{}", &item)}><li>{item.to_string()}</li></a> }
                    }).collect_view()
                },
             }}
        </ul>
    }
}
