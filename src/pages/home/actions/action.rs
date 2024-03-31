use crate::components::graph::{GraphEdge, GraphNode, GraphPipe, GraphView, PipeDirection};
use crate::utils::color::ThemeColor;
use leptos::logging::log;
use leptos::*;
use shared::models::app::VulpineAction;

#[component]
pub fn ActionDialog(
    #[prop(into)] action: Signal<Option<VulpineAction>>,
    #[prop(into)] on_close: Callback<VulpineAction>,
) -> impl IntoView {
    let current_action = create_rw_signal(VulpineAction::default());
    let current_position = create_rw_signal((0, 0));
    let nodes = create_rw_signal::<Vec<GraphNode>>(Vec::new());
    let edges = create_rw_signal::<Vec<GraphEdge>>(Vec::new());

    create_effect(move |_| {
        current_action.set(action.get().unwrap_or_default());
    });

    let add_node = move |_| {
        nodes.update(|n| {
            let (x, y) = current_position.get_untracked();
            log!("Node added at {}, {}", x, y);
            n.push(GraphNode {
                id: n.len().to_string(),
                x,
                y,
                pipes: vec! [
                    GraphPipe {
                        id: "0".to_string(),
                        name: "input".to_string(),
                        direction: PipeDirection::Input,
                        color: ThemeColor::Yellow,
                    },
                    GraphPipe {
                        id: "0".to_string(),
                        name: "output".to_string(),
                        direction: PipeDirection::Output,
                        color: ThemeColor::Purple,
                    },
                    GraphPipe {
                        id: "0".to_string(),
                        name: "both".to_string(),
                        direction: PipeDirection::Both,
                        color: ThemeColor::Blue,
                    },
                ],
            });
        });
    };
    let on_node_move = move |(id, x, y)| {
        nodes.update(|n| {
            if let Some(node) = n.iter_mut().find(|n| n.id == id) {
                node.x = x;
                node.y = y;
            }
        });
    };

    view! {
        <Show when={move || action.get().is_some()}>
            <div class="fullscreen col gap-sm layout-container">
                <div class="row align-center gap-sm card paper pv-xs ph-md">
                    <h2 class="flex mp-none">
                        "Edit action"
                    </h2>
                    <button class="btn primary p-xs" on:click={move |_| on_close.call(current_action.get())}>
                        <i class="ph-light ph-x text-icon"/>
                    </button>
                </div>
                <div class="flex sidebar-layout">
                    <div class="min-w-sm card paper">
                        <h2>"Add"</h2>
                        <button class="card primary" on:click=add_node>
                            "Node"
                        </button>
                    </div>
                    <div class="flex card paper min-w-ws min-h-md">
                        <GraphView nodes edges build_node={|e| {
                            view! {
                                <p class="w-md">{format!("{:?}", e)}</p>
                            }
                        }} current_position on_node_move />
                    </div>
                </div>
            </div>
        </Show>
    }
}
