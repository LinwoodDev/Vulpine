use leptos::*;

#[component]
pub fn ResorcesAppView() -> impl IntoView {
    view! {
        <div class="col card paper p-sm gap-xs">
            {move || format!("Exes: {:?}", app.get().executables.len())}
            <For each={move || app.get().executables.clone()} key={|(key, _)| key.to_string()}
                children={move |(name, exe)| {
                    let id = store_value(name.to_string());
                    view! {
                        <div class="card fill col">
                            <div class="row justify-between align-center">
                                <strong>{name}</strong>
                                <Show when={move || edit.get()}>
                                    <button class="btn p-xs" on:click={move |_| {
                                        let id = id.get_value().to_string();
                                        app.update(move |app| {
                                            app.executables.remove(&id);
                                        });
                                    }}>
                                        <img class="invert icon" title="Remove" src="/public/icons/trash-light.svg" alt="Trash icon"/>
                                    </button>
                                </Show>
                            </div>
                            <div class="form-group">
                                <label for="env-description">"Description"</label>
                                <input type="text" id="env-description"
                                    readonly={move || !edit.get()}
                                    prop:value={move || exe.description.to_string()}
                                    on:input={move |ev| {
                                        let description = event_target_value(&ev);
                                        let id = id.get_value().to_string();
                                        app.update(move |app| {
                                            app.executables.entry(id)
                                                .and_modify(|exe| exe.description = description);
                                        });
                                    }} />
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
    }
}
