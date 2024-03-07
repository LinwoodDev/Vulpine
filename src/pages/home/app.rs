use leptos::*;
use shared::models::app::VulpineApp;

#[component]
pub fn HomeAppView(app: RwSignal<VulpineApp>, #[prop(into)] edit: MaybeSignal<bool>) -> impl IntoView {
    const TABS: [&str; 3] = ["Tab 1", "Tab 2", "Tab 3"];
    view! {<div class="col gap-xs flex h-full">
        {move || format!("Edit: {:?}", edit.get())}
        <div class="form-group">
            <label for="name">"Name"</label>
            <input type="text" id="name"
                readonly={move || !edit.get()}
                prop:value={move || app.get().name}
                on:input={move |ev| {
                    let name = event_target_value(&ev);
                    app.update(|app| app.name = name);
                }} />
        </div>
        <div class="form-group">
            <label for="version">"Version"</label>
            <input type="text" id="version"
                readonly={move || !edit.get()}
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
                readonly={move || !edit.get()}
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
