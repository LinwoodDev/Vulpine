use leptos::*;
use crate::components::titlebar::Titlebar;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="col h-full no-overflow">
            <Titlebar />
            <main class="flex">{children()}</main>
        </div>
    }
}
