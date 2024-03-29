use leptos::*;
use shared::models::app::{VulpineApp, VulpineResource};

use crate::components::accordion::{Accordion, AccordionItem, AccordionItemContent, AccordionItemTitle};

#[component]
pub fn ResorcesAppView(
    app: RwSignal<VulpineApp>,
    #[prop(into)] edit: MaybeSignal<bool>,
) -> impl IntoView {
    let adding_executable_name = create_rw_signal::<String>(String::new());
    let current_executable = create_rw_signal::<Option<String>>(None);
    let on_executable_add = store_value(move || {
        let name = adding_executable_name.get_untracked();
        app.update(|app| {
            app.resources.insert(
                name.clone(),
                VulpineResource::default(),
            );
        });
        current_executable.set(Some(name));
        adding_executable_name.set(String::new());
    });
    view! {
        <div class="col p-sm gap-xs container-md w-full">
            {move || format!("Exes: {:?}", app.get().resources.len())}
            <Accordion value=current_executable on_change={move |e| current_executable.set(e)}>
                <For each={move || app.get().resources.clone()} key={|(key, _)| key.to_string()}
                    children={move |(name, exe)| {
                        let id = store_value(name.to_string());
                        let description = store_value(exe.description.to_string());
                        view! {
                            <AccordionItem key=name.to_string()>
                                <AccordionItemTitle title=&name>
                                    <Show when={move || edit.get()}>
                                        <button class="btn p-xs" on:click={move |_| {
                                            let id = id.get_value().to_string();
                                            app.update(move |app| {
                                                app.resources.shift_remove(&id);
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
                                                    app.resources.entry(id)
                                                        .and_modify(|exe| exe.description = description);
                                                });
                                            }} />
                                    </div>
                                </AccordionItemContent>
                            </AccordionItem>
                        }
                    }
                } />
            </Accordion>
            <Show when={move || edit.get()}>
                <hr />
                <div class="card paper row gap-xs ph-xs align-center">
                    <input type="text" class="flex" prop:value={adding_executable_name.clone()} on:input={move |ev| adding_executable_name.set(event_target_value(&ev))} on:change={move |_| on_executable_add.get_value()()} />
                    <button class="btn secondary row p-xs" on:click={move |_| on_executable_add.get_value()()}>
                        <i class="ph-light ph-plus text-icon"/>
                    </button>
                </div>
            </Show>
        </div>
    }
}
