use leptos::*;
use shared::models::app::VulpineApp;

use crate::pages::home::actions::ActionsAppView;

#[component]
pub fn GeneralAppView(
    app: RwSignal<VulpineApp>,
    #[prop(into)] edit: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when={move || !edit.get()}>
            <ActionsAppView app={app.clone()} edit={edit.clone()} />
        </Show>
        <div class="col">
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
        </div>
    }
}
