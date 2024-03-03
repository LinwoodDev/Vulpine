use leptos::*;
use leptos_router::*;

#[component]
pub fn ActionPage() -> impl IntoView {
    let params = use_params_map();
    let id = create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));
    view! {<HomePage action_id={id} />}
}

#[component]
pub fn HomePage(#[prop(optional, into)] action_id : MaybeProp<String>) -> impl IntoView {
    let cloned_id = action_id.clone();
    let only_large = create_memo(move |_| cloned_id.get().is_none());
    view! {
        <div class="sidebar-layout">
            <HomeListView only_large={only_large} />
            <HomeDetailsView id={action_id} />
        </div>
    }
}

#[component]
fn HomeListView(#[prop(optional, into)] only_large : MaybeSignal<bool>) -> impl IntoView {
    const ITEMS: [&str; 3] = ["Item 1", "Item 2", "Item 3"];
    view! {
        <ul class="col min-w-md gap-xs mh-xs" class:show_xs={only_large}>
            <li class="row justify-between align-center ph-xs">
                <h2 class="bold"><a href="/" class="no-decoration normal-color">Actions</a></h2>
                <div class="row gap-xs">
                    <a href="/actions" class="btn secondary p-xs"><img class="invert icon" title="Add" src="public/icons/plus-light.svg" alt="Plus icon"/></a>
                    <a href="/settings" class="btn secondary p-xs"><img class="invert icon" title="Settings" src="public/icons/gear-light.svg" alt="Gear icon"/></a>
                </div>
            </li>
            { ITEMS.iter().map(|item| view! { <a class="card primary no-decoration bold" href={format!("/actions/{}", item)}><li>{item.to_string()}</li></a> }).collect_view() }
        </ul>
    }
}

#[component]
fn HomeDetailsView(#[prop(optional_no_strip, into)] id : MaybeProp<String>) -> impl IntoView {
    let cloned_id = id.clone();
    let show = move || cloned_id.get().is_none();
    let cloned_id = id.clone();
    let title = move || {
        let id = cloned_id.clone().get().unwrap_or_default();
        if id.is_empty() { "Add".to_string() } else { "Edit".to_string() }
    };
    view! {
        <div class="col card paper flex ph-sm min-w-md" class:show-xs={show()}>
        <Show when={move || !show()}>
            <div class="row align-center gap-sm">
                <h2 class="bold">
                    {title()}
                </h2>
                <input type="text" class="flex" />
                <button class="btn primary">"Save"</button>
            </div>
        </Show>
        <div class="flex col justify-center">
        {move || match id.clone().get() {
            Some(action) => view! {<p>"Selected " {action}</p>},
            None => view! {<p class="text-center">"Nothing selected"</p>}
        }}
        </div>
        </div>
    }
}
