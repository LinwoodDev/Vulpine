use leptos::{logging::log, *};
use web_sys::MouseEvent;

use crate::utils::color::ThemeColor;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub x: i32,
    pub y: i32,
    pub id: String,
    pub pipes: Vec<GraphPipe>,
}

#[derive(Debug, Clone)]
pub struct GraphPipe {
    pub name: String,
    pub id: String,
    pub color: ThemeColor,
    pub direction: PipeDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeDirection {
    Input,
    Output,
    Both,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub from: String,
    pub from_pipe: Option<String>,
    pub to: String,
    pub to_pipe: Option<String>,
}

#[component]
pub fn GraphView<FN, N>(
    #[prop(into)] nodes: MaybeSignal<Vec<GraphNode>>,
    #[prop(into)] edges: MaybeSignal<Vec<GraphEdge>>,
    build_node: FN,
    current_position: RwSignal<(i32, i32)>,
    #[prop(into, optional)] on_node_move: Option<Callback<(String, i32, i32)>>,
) -> impl IntoView
where
    FN: Fn(&GraphNode) -> N + 'static,
    N: IntoView + 'static,
{
    let dragging_id = create_rw_signal::<Option<String>>(None);
    let dragging_start = create_rw_signal::<Option<(i32, i32)>>(None);
    let on_down = move |e: MouseEvent| {
        if dragging_id.get_untracked().is_some() {
            return;
        }
        let last = current_position.get_untracked();
        dragging_start.set(Some((last.0 + e.page_x(), e.page_y() + last.1)));
    };
    let up_handle = window_event_listener(ev::mouseup, move |_| {
        dragging_start.set(None);
        dragging_id.set(None);
    });
    let move_handle = window_event_listener(ev::mousemove, move |e| {
        let start = dragging_start.get_untracked();
        let Some((start_x, start_y)) = start else {
            return;
        };
        let offset_x = e.page_x();
        let offset_y = e.page_y();
        let x = start_x - offset_x;
        let y = start_y - offset_y;
        let Some(dragging_id) = dragging_id.get_untracked() else {
            current_position.update(|last| {
                last.0 = x;
                last.1 = y;
            });
            return;
        };
        on_node_move.map(|cb| {
            cb.call((dragging_id.clone(), -x, -y));
        });
    });
    on_cleanup(move || {
        up_handle.remove();
        move_handle.remove();
    });
    view! {
        <div class="w-full h-full view no-overflow" on:mousedown=on_down>
            <svg width="100%" height="100%">
                <pattern id="background-pattern" x={move || -current_position.get().0} y={move || -current_position.get().1} width="32" height="32" patternUnits="userSpaceOnUse">
                    <circle cx="14" cy="14" r="4" fill="rgba(255,255,255,0.1)"/>
                </pattern>
                <rect width="100%" height="100%" fill="url(#background-pattern)" />
            </svg>
            <div class="absolute-full no-overflow">
                    <div style={move || {
                        let pos = current_position.get();
                        format!("transform: translate({}px, {}px);", -pos.0, -pos.1)
                    }} class="w-full h-full">
                        <For each={move || nodes.get()} key={|e| format!("{:?}", e)} children=move |node| {
                            let id = store_value(node.id.clone());
                            let on_node_down = move |e: MouseEvent| {
                                e.prevent_default();
                                let start = (e.page_x() -node.x, e.page_y()-node.y);
                                dragging_start.set(Some(start));
                                dragging_id.set(Some(id.get_value().clone()));
                            };
                            let on_input_down = move |e: MouseEvent| {
                                log!("Input down");
                            };
                            let on_output_down = move |e: MouseEvent| {
                                log!("Output down");
                            };
                            let pipes = store_value(node.pipes.clone());
                            view! {
                                <div class="card paper w-max gap-none" style={move || format!("position: absolute; left: {}px; top: {}px;", node.x, node.y)} on:mousedown=on_node_down>
                                    {build_node(&node)}
                                    <hr />
                                    <div class="col">
                                        <For each={move || {
                                            let pipes = pipes.get_value();
                                            pipes
                                        }} key={|e| format!("{:?}", e)} children=move |pipe| {
                                            view! {
                                                <GraphPipe pipe on_input_down on_output_down />
                                            }
                                        }/>
                                    </div>
                                </div>
                            }
                        } />
                    </div>
            </div>
        </div>
    }
}

#[component]
pub fn GraphPipe(
    pipe: GraphPipe,
    #[prop(into)] on_input_down: Callback<MouseEvent>,
    #[prop(into)] on_output_down: Callback<MouseEvent>,
) -> impl IntoView {
    let bg_class = store_value(pipe.color.to_bg_class());
    let direction = store_value(pipe.direction.clone());
    let input_down = move |e| {
        if direction.get_value() != PipeDirection::Output {
            on_input_down.call(e);
        }
    };
    let output_down = move |e| {
        if direction.get_value() != PipeDirection::Input {
            on_output_down.call(e);
        }
    };
    view! {
        <div class="row gap-xs align-center justify-between">
            <div on:mousedown=input_down class={move || format!("m-sm dot {}", if direction.get_value() == PipeDirection::Output { "".to_owned() } else { bg_class.get_value().clone() })} />
            <p>{pipe.name}</p>
            <div on:mousedown=output_down class={move || format!("m-sm dot {}", if direction.get_value() == PipeDirection::Input { "".to_owned() } else { bg_class.get_value().clone() })} />
        </div>
    }
}
