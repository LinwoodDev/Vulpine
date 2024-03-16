use leptos::{html::Dialog, *};
use web_sys::MouseEvent;

#[component]
pub fn Dialog(
    children: ChildrenFn,
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(into)] show: Signal<bool>,
    #[prop(optional, into)] on_close: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<Dialog>();
    create_effect(move |_| {
        let show = show.get();
        input_ref.on_load(move |dialog| {
            if show {
                let _ = dialog.show_modal();
            } else {
                dialog.close();
            }
        });
    });
    let on_close_callback = on_close.unwrap_or(Callback::new(|_| {}));
    let title = store_value(title);
    let children = store_value(children);
    view! {
        <Portal>
            <dialog class="dialog" display="flex" _ref=input_ref>
                <div class="dialog-title">
                    {move || title.get_value().get().to_string()}
                    <Show when={move || on_close.is_some()}>
                        <button class="btn" on:click={move |e| {
                            e.prevent_default();
                            on_close_callback.call(e);
                        }}>{"Close"}</button>
                    </Show>
                </div>
                <div class="dialog-content">
                    {children.with_value(|children| children())}
                </div>
            </dialog>
        </Portal>
    }
}
