use leptos::*;
use shared::models::app::VulpineAction;
use crate::components::graph::{GraphNode, GraphView};

#[component]
pub fn ActionDialog(#[prop(into)] action: Signal<Option<VulpineAction>>, #[prop(into)] on_close: Callback<VulpineAction>) -> impl IntoView {
    let current_action = create_rw_signal(VulpineAction::default());

    create_effect(move |_| {
        current_action.set(action.get().unwrap_or_default());
    });
    
    view! {
        <Show when={move || action.get().is_some()}>
            <div class="fullscreen col gap-sm">
                <div class="row align-center gap-sm card paper pv-xs ph-md">
                    <h2 class="flex mp-none">
                        "Edit action"
                    </h2>
                    <button class="btn primary p-xs" on:click={move |_| on_close.call(current_action.get())}>
                        <i class="ph-light ph-x text-icon"/>
                    </button>
                </div>
                <div class="flex card paper">
                    <GraphView nodes={vec! [GraphNode {
                        x: 100,
                        y: 100,
                        id: 0,
                    }]} build_node={|e| {
                        view! {
                            <p>{format!("{:?}", e)}</p>
                        }
                    }} />
                </div>
            </div>
        </Show>
    }
}
