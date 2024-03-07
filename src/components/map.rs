use leptos::*;

#[component]
pub fn MapField<T>(#[prop(into)] value : MaybeSignal<T>, #[prop(into, optional_no_strip)] on_change : Option<fn(String, String) -> ()>, #[prop(into, optional_no_strip)] on_add : Option<fn(String) -> ()>, #[prop(optional, into)] title: Option<String>) -> impl IntoView
where
    T: Iterator<Item = (String, String)> + Clone + 'static,
{
    let title = store_value(title);
    let adding_key = create_rw_signal(String::new());
    view! {
        <div class="col card paper">
            <Show when={move || title.get_value().is_some()}>
                <h3>{move || title.get_value().unwrap_or_default()}</h3>
            </Show>
            { move || value.get().map(|(key, value)| view! {
                <div class="row gap-xs">
                    <input class="flex" type="text" readonly={move || on_change.is_none()} prop:value={key} />
                    <input class="flex" type="text" readonly={move || on_change.is_none()} prop:value={value} />
                </div>
            }).collect_view() }
            <Show when={move || on_add.is_some()}>
                <div class="row gap-xs">
                    <input class="flex" type="text" readonly={false} prop:value={adding_key.get()} on:input={move |ev| adding_key.set(event_target_value(&ev))} />
                    <button class="btn p-xs" on:click={move |_| on_add.unwrap()(adding_key.get())}>{"Add"}</button>
                </div>
            </Show>
        </div>
    }
}
