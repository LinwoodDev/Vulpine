use leptos::*;
use shared::models::app::{VulpineAction, VulpineApp, VulpineExecutable};

#[component]
pub fn HomeAppView(
    app: RwSignal<VulpineApp>,
    #[prop(into)] edit: MaybeSignal<bool>,
) -> impl IntoView {
    let adding_executable_name = create_rw_signal::<String>(String::new());
    let on_executable_add = move |_| {
        app.update(|app| {
            app.executables.insert(
                adding_executable_name.get_untracked(),
                VulpineExecutable::default(),
            );
        });
        adding_executable_name.set(String::new());
    };
    let adding_action_name = create_rw_signal::<String>(String::new());
    let on_action_add = move |_| {
        app.update(|app| {
            app.actions
                .insert(adding_action_name.get_untracked(), VulpineAction::default());
        });
        adding_action_name.set(String::new());
    };

    let action_name = create_rw_signal::<String>(String::new());
    let action = create_memo(move |_| app.get().actions.get(&action_name.get()).cloned());
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
            <h3>"Actions"</h3>
            <div class="overflow-x row gap-xs align-center justify-start">
                { move || app.get().actions.iter().map(|(name, _)| {
                    let stored_name = store_value(name.to_string());
                    let on_delete = move |_| {
                        app.update(move |app| {
                            app.actions.remove(&stored_name.get_value());
                        });
                    };
                    view! {
                        <button class="btn secondary">
                            <div class="row gap-xs align-center justify-between">
                                <p class="m-none">{name.to_string()}</p>
                                <Show when={move || edit.get()}>
                                    <button class="btn p-none min-w-max" on:click=on_delete>
                                        <img class="invert icon" title="Delete" src="/public/icons/trash-light.svg" alt="Trash icon"/>
                                    </button>
                                </Show>
                            </div>
                        </button>
                    }
                }).collect_view()}
                <Show when={move || edit.get()}>
                    <div class="card paper row gap-xs ph-xs min-w-sm">
                        <input type="text" prop:value={adding_action_name.clone()} on:input={move |ev| adding_action_name.set(event_target_value(&ev))} />
                        <button href="#" class="btn p-xs" on:click=on_action_add>
                            <img class="invert icon" title="Add" src="/public/icons/plus-light.svg" alt="Plus icon"/>
                        </button>
                    </div>
                </Show>
            </div>
            <Show when={move || action.get().is_some()}>
                <div class="card paper">
                    {move || if let Some(action) = action.get() {
                        view! {
                            <div class="col gap-xs">
                                <h3>{action_name.clone()}</h3>
                                <div class="form-group">
                                    <label for="action-description">"Description"</label>
                                    <input type="text" id="action-description"
                                        readonly={move || !edit.get()}
                                        prop:value={move || action.description.to_string()}
                                        on:input={move |ev| {
                                            let description = event_target_value(&ev);
                                            app.update(move |app| {
                                                app.actions.entry(action_name.get_untracked())
                                                    .and_modify(|action| action.description = description);
                                            });
                                        }} />
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {<p>"No action selected"</p>}.into_any()
                    }}
                </div>
            </Show>
            Value: {move || format!("{:?}", app.get())}
        </div>
    }
}
