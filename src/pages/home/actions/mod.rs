use leptos::*;
use shared::models::app::{VulpineAction, VulpineApp};

use crate::components::accordion::{Accordion, AccordionItem, AccordionItemContent, AccordionItemTitle};

#[component]
pub fn ActionsAppView(
    app: RwSignal<VulpineApp>,
    #[prop(into)] edit: MaybeSignal<bool>,
) -> impl IntoView {
    let adding_action_name = create_rw_signal::<String>(String::new());
    let current_action = create_rw_signal::<Option<String>>(None);
    let on_action_add = store_value(move || {
        let name = adding_action_name.get_untracked();
        app.update(|app| {
            app.actions.insert(
                name.clone(),
                VulpineAction::default(),
            );
        });
        current_action.set(Some(name));
        adding_action_name.set(String::new());
    });
    view! {
        <Show when={move || !edit.get()}>
            <div class="overflow-x row gap-xs align-center justify-start">
                { move || app.get().actions.iter().map(|(name, _)| {
                    let stored_name = store_value(name.to_string());
                    let on_delete = move |_| {
                        app.update(move |app| {
                            app.actions.shift_remove(&stored_name.get_value());
                        });
                    };
                    view! {
                        <button class="btn secondary">
                            <div class="row gap-xs align-center justify-between">
                                <p class="m-none">{name.to_string()}</p>
                                <Show when={move || edit.get()}>
                                    <button class="btn p-none min-w-max row p-sm" on:click=on_delete>
                                        <i class="ph-light ph-trash text-icon"/>
                                    </button>
                                </Show>
                            </div>
                        </button>
                    }
                }).collect_view()}
            </div>
        </Show>
        <Show when={move || edit.get()}>
            <div class="col p-sm gap-xs container-md w-full">
                {move || format!("Actions: {:?}", app.get().actions.len())}
                <Accordion value=current_action on_change={move |e| current_action.set(e)}>
                    <For each={move || app.get().actions.clone()} key={|(key, _)| key.to_string()}
                        children={move |(name, action)| {
                            let id = store_value(name.to_string());
                            let description = store_value(action.description.to_string());
                            view! {
                                <AccordionItem key=name.to_string()>
                                    <AccordionItemTitle title=&name>
                                        <Show when={move || edit.get()}>
                                            <button class="btn p-xs" on:click={move |_| {
                                                let id = id.get_value().to_string();
                                                app.update(move |app| {
                                                    app.actions.shift_remove(&id);
                                                });
                                            }}>
                                                <i class="ph-light ph-trash text-icon"/>
                                            </button>
                                        </Show>
                                    </AccordionItemTitle>
                                    <AccordionItemContent>
                                        <div class="form-group">
                                            <label for="env-description">"Description"</label>
                                            <input type="text" id="env-description"
                                                readonly={move || !edit.get()}
                                                prop:value={move || description.get_value()}
                                                on:input={move |ev| {
                                                    let description = event_target_value(&ev);
                                                    let id = id.get_value().to_string();
                                                    app.update(move |app| {
                                                        app.actions.entry(id)
                                                            .and_modify(|action| action.description = description);
                                                    });
                                                }} />
                                        </div>
                                    </AccordionItemContent>
                                </AccordionItem>
                            }
                        }
                    } />
                </Accordion>
                <hr />
                <div class="card paper row gap-xs ph-xs align-center">
                    <input type="text" class="flex" prop:value={adding_action_name.clone()} on:input={move |ev| adding_action_name.set(event_target_value(&ev))} on:change={move |_| on_action_add.get_value()()} />
                    <button class="btn secondary row p-xs" on:click={move |_| on_action_add.get_value()()}>
                        <i class="ph-light ph-plus text-icon"/>
                    </button>
                </div>
            </div>
        </Show>
    }
}
