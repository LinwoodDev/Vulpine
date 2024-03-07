use leptos::*;
use shared::models::app::{VulpineApp, VulpineExecutable};

#[component]
pub fn HomeAppView(app: RwSignal<VulpineApp>, #[prop(into)] edit: MaybeSignal<bool>) -> impl IntoView {
    let adding_executable_name = create_rw_signal::<String>(String::new());
    let on_executable_add = move |_| {
        app.update(|app| {
            app.executables.insert(adding_executable_name.get_untracked(), VulpineExecutable::default());
        });
    };
    view! {
        <div class="col gap-xs flex h-full">
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
            <h3>"Executables"</h3>
            <div class="col card paper p-sm gap-xs">
                {move || format!("Exes: {:?}", app.get().executables.len())}
                <For each={move || app.get().executables.clone()} key={|(key, _)| key.to_string()}
                    children={move |(name, exe)| {
                        let id = name.to_string();
                        view! {
                            <div class="card fill row">
                                <div class="col">
                                    <strong>{name}</strong>
                                    <div class="form-group">
                                        <label for="env-description">"Description"</label>
                                        <input type="text" id="env-description"
                                            readonly={move || !edit.get()}
                                            prop:value={move || exe.description.to_string()}
                                            on:input={move |ev| {
                                                let description = event_target_value(&ev);
                                                let id = id.to_string();
                                                app.update(move |app| {
                                                    app.executables.entry(id)
                                                        .and_modify(|exe| exe.description = description);
                                                });
                                            }} />
                                    </div>
                                </div>
                            </div>
                        }
                    }
                } />
                <Show when={move || edit.get()}>
                    <hr />
                    <div class="row">
                        <input type="text" class="flex" prop:value={adding_executable_name.clone()} on:input={move |ev| adding_executable_name.set(event_target_value(&ev))} />
                        <button class="btn p-xs" on:click=on_executable_add>
                            <img class="invert icon" title="Add" src="/public/icons/plus-light.svg" alt="Plus icon"/>
                        </button>
                    </div>
                </Show>
            </div>
            <h3>"Actions"</h3>
            <div class="overflow-x row gap-xs align-center">
                { move || app.get().actions.iter().map(|(name, _)| view! {<a href="#" class="btn secondary p-xs">{name.to_string()}</a>}).collect_view() }
                <Show when={move || edit.get()}>
                    <button href="#" class="btn p-xs" on:click=on_executable_add>
                        <img class="invert icon" title="Add" src="/public/icons/plus-light.svg" alt="Plus icon"/>
                    </button>
                </Show>
            </div>
            <div class="card paper">
            </div>
            Value: {move || format!("{:?}", app.get())}
        </div>
    }
}
