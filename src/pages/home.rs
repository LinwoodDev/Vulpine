use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="sidebar-layout">
            <HomeListView />
            <HomeDetailsView />
        </div>
    }
}

#[component]
fn HomeListView() -> impl IntoView {
    const ITEMS: [&str; 3] = ["Item 1", "Item 2", "Item 3"];
    view! {
        <ul class="col min-w-md gap-xs mh-xs">
            <li class="row justify-between align-center ph-xs">
                <h2 class="bold">Actions</h2>
                <a class="btn secondary">"Add"</a>
            </li>
            { ITEMS.iter().map(|item| view! { <a class="card primary no-decoration bold" href="/test"><li>{item.to_string()}</li></a> }).collect_view() }
        </ul>
    }
}

#[component]
fn HomeDetailsView() -> impl IntoView {
    view! {
        <div class="col card paper flex ph-sm min-w-md">
        <h2 class="bold">"Edit"</h2>
        <div class="flex col justify-center">
        <p class="text-center">"No item selected"</p>
        </div>
        </div>
    }
}
