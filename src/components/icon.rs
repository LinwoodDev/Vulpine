use leptos::*;

#[component]
pub fn Icon(#[prop(into)] path: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 64 64">
            <use_ href={format!("{}#icon", path)} />
        </svg>
    }
}