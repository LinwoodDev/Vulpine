use leptos::*;
use shared::models::app::{VulpineAction, VulpineApp};

#[component]
pub fn ActionsAppView(
    app: RwSignal<VulpineApp>,
    #[prop(into)] edit: MaybeSignal<bool>,
) -> impl IntoView {
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
                <div class="card paper row gap-xs ph-xs">
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
    }
}
