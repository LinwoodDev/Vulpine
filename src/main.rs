mod api;
mod app;
mod components;
mod layouts;
mod pages;
mod utils;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
