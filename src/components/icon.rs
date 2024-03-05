use leptos::*;

#[component]
pub fn Icon(#[prop(into)] src: String, #[prop(optional, into)] title: Option<String>, #[prop(optional, into)] alt: Option<String>) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32" aria-label={alt}>
            {if let Some(text) = title {
                view! {<title>{text}</title>}.into_view()
            } else {
                view! {}.into_view()
            }}
            <use_ data={format!("{}", src)} type="image/svg+xml" />
        </svg>
    }
}