use leptos::*;

#[component]
pub fn ActionView(#[prop(into)] action: Signal<Option<VulpineAction>>, on_close: Callback<VulpineAction>) -> impl IntoView {
    view! {
        <Dialog title="Edit action" show={show_add} on_close=on_close>
        
        </Dialog>
    }
}
