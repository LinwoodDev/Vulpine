use std::collections::HashMap;

use leptos::{html::Div, logging::log, *};
use web_sys::PointerEvent;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: String,
    pub from_pipe: String,
    pub to: String,
    pub to_pipe: String,
}

#[derive(Debug, Clone)]
pub struct CurrentConnection {
    pub from: String,
    pub from_pipe: String,
    pub is_input: bool,
    pub from_position: (f64, f64),
    pub current_position: Option<(f64, f64)>,
}

#[derive(Clone, Default)]
struct GraphContext {
    pub pipe_positions: RwSignal<HashMap<(String, String), ((f64, f64), (f64, f64))>>,
}

const DOT_SIZE: f64 = 16_f64;

fn is_near_dot((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> bool {
    (x1 - x2).abs() <= DOT_SIZE && (y1 - y2).abs() <= DOT_SIZE
}

#[component]
pub fn GraphView<FN, N>(
    #[prop(into)] nodes: MaybeSignal<Vec<GraphNode>>,
    #[prop(into)] edges: MaybeSignal<Vec<GraphEdge>>,
    build_node: FN,
    current_position: RwSignal<(i32, i32)>,
    #[prop(into, optional)] on_node_move: Option<Callback<(String, i32, i32)>>,
    #[prop(into, optional)] on_edge_add: Option<Callback<((String, String), (String, String))>>,
) -> impl IntoView
where
    FN: Fn(&GraphNode) -> N + 'static,
    N: IntoView + 'static,
{
    let canvas_ref: NodeRef<Div> = create_node_ref();
    let pipe_positions =
        create_rw_signal::<HashMap<(String, String), ((f64, f64), (f64, f64))>>(HashMap::new());
    provide_context(GraphContext { pipe_positions });
    let local_to_global = move |x: i32, y: i32| {
        let Some(element) = canvas_ref.get() else {
            return (x as f64, y as f64);
        };
        let rect = element.get_bounding_client_rect();
        (x as f64 - rect.left(), y as f64 - rect.top())
    };
    let get_path = move |(x1, y1): (f64, f64), (x2, y2): (f64, f64)| {
        let ctrl_x1 = x1 + (x2 - x1) / 4.0;
        let ctrl_y1 = y1;
        let ctrl_x2 = x1 + 3.0 * (x2 - x1) / 4.0;
        let ctrl_y2 = y2;
        format!(
            "M {} {} C {} {} {} {} {} {}",
            x1, y1, ctrl_x1, ctrl_y1, ctrl_x2, ctrl_y2, x2, y2
        )
    };
    let dragging_id = create_rw_signal::<Option<String>>(None);
    let dragging_start = create_rw_signal::<Option<(i32, i32)>>(None);
    let current_connection = create_rw_signal::<Option<CurrentConnection>>(None);
    let on_down = move |e: PointerEvent| {
        e.prevent_default();
        let last = current_position.get_untracked();
        let pos = (last.0 + e.page_x(), e.page_y() + last.1);
        dragging_start.update(|e| {
            let Some(e) = e else {
                *e = Some(pos);
                return;
            };
            e.0 += pos.0;
            e.1 += pos.1;
        });
    };
    let up_handle = window_event_listener(ev::pointerup, move |e| {
        e.prevent_default();
        if let Some(con) = current_connection.get_untracked() {
            on_edge_add.map(|f| {
                let mut first = (con.from.to_string(), con.from_pipe.to_string());
                let Some(mut second) = ({
                    let pipe_positions = pipe_positions.get();
                    let Some(mouse) = con.current_position else {
                        return;
                    };
                    let entry = pipe_positions.iter().find(|(_, e)| is_near_dot(e.0,mouse)|| is_near_dot(e.1,mouse));
                    entry.map(|e| (e.0.0.to_string(), e.0.1.to_string()))
                }) else {
                    return;
                };
                if con.is_input {
                    (first, second) = (second, first);
                }
                f.call((first, second))
            });
        }
        current_connection.set(None);
        dragging_start.set(None);
        dragging_id.set(None);
    });
    let move_handle = window_event_listener(ev::pointermove, move |e| {
        let start = dragging_start.get_untracked();
        let Some((start_x, start_y)) = start else {
            return;
        };
        e.prevent_default();
        let offset_x = e.page_x();
        let offset_y = e.page_y();
        let x = start_x - offset_x;
        let y = start_y - offset_y;
        if current_connection.get_untracked().is_some() {
            current_connection.update(|f| {
                let Some(o) = f.as_mut() else {
                    return;
                };
                let mouse_pos = local_to_global(e.page_x(), e.page_y());
                o.current_position = Some(mouse_pos);
            });
            return;
        }
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
        <div class="w-full h-full view no-overflow" on:pointerdown=on_down>
            <svg width="100%" height="100%">
                <pattern id="background-pattern" x={move || -current_position.get().0} y={move || -current_position.get().1} width="32" height="32" patternUnits="userSpaceOnUse">
                    <circle cx="14" cy="14" r="4" fill="rgba(255,255,255,0.1)"/>
                </pattern>
                <rect width="100%" height="100%" fill="url(#background-pattern)" />
            </svg>
            <div class="absolute-full no-overflow">
                    <div _ref=canvas_ref style={move || {
                        let pos = current_position.get();
                        format!("transform: translate({}px, {}px);", -pos.0, -pos.1)
                    }} class="w-full h-full">
                        <For each={move || nodes.get()} key={|e| format!("{:?}", e)} children=move |node| {
                            let id = store_value(node.id.clone());
                            let node_ref : NodeRef<Div> = create_node_ref();
                            let node_position = create_memo(move |_| {
                                let Some(node_ref) = node_ref.get() else {
                                    return (0_f64, 0_f64);
                                };
                                let rect = node_ref.get_bounding_client_rect();
                                (
                                    rect.x() - node.x as f64,
                                    rect.y() - node.y as f64
                                )
                            });
                            let on_node_down = move |e: PointerEvent| {
                                e.prevent_default();
                                if current_connection.get_untracked().is_some() {
                                    return;
                                }
                                let current_position = current_position.get();
                                let start = (-node.x - current_position.0, -node.y -current_position.1);
                                dragging_start.set(Some(start));
                                dragging_id.set(Some(id.get_value().clone()));
                            };
                            let change_current_connection = move |e: PointerEvent, pipe: &GraphPipe, is_input: bool| {
                                log!("Current connection change");
                                let mouse_pos = local_to_global(e.page_x(), e.page_y());
                                current_connection.set(Some(CurrentConnection {
                                    from_pipe: pipe.id.to_string(),
                                    from: id.get_value().clone(),
                                    current_position: None,
                                    from_position: mouse_pos,
                                    is_input
                                }));
                            };
                            let pipes = store_value(node.pipes.clone());
                            view! {
                                <div _ref=node_ref class="card paper w-max gap-none" style={move || format!("position: absolute; left: {}px; top: {}px;", node.x, node.y)} on:pointerdown=on_node_down>
                                    {build_node(&node)}
                                    <hr />
                                    <div class="col">
                                        <For each={move || {
                                            let pipes = pipes.get_value();
                                            pipes
                                        }} key={|e| format!("{:?}", e)} children=move |pipe| {
                                            let pipe = store_value(pipe);
                                            let change_current_connection = store_value(change_current_connection);
                                            let on_input_down = move |e: PointerEvent| {
                                                change_current_connection.get_value()(e, &pipe.get_value(), true);
                                            };
                                            let on_output_down = move |e: PointerEvent| {
                                                change_current_connection.get_value()(e, &pipe.get_value(), false);
                                            };
                                            view! {
                                                <GraphPipe node_position id=id.get_value() pipe=pipe.get_value() on_input_down=on_input_down on_output_down=on_output_down />
                                            }
                                        }/>
                                    </div>
                                </div>
                            }
                        } />
                        <svg width="100%" height="100%" style="overflow: visible">
                            <path d={move || {
                                let Some(con) = current_connection.get() else {
                                    return "".to_owned();
                                };
                                let Some(current) = con.current_position else {
                                    return "".to_owned();
                                };
                                get_path(con.from_position, current)
                            }} stroke="white" fill="transparent" />
                            <For each={move || edges.get()} key={|e| format!("{:?}", e)} children=move |edge| {
                                let edge = store_value(edge);
                                let get_pos = move |from: bool| {
                                    let pipe_positions = pipe_positions.get();
                                    let edge = edge.get_value();
                                    let key = if from {
                                        (edge.from, edge.from_pipe)
                                    } else {
                                        (edge.to, edge.to_pipe)
                                    };
                                    let entry = pipe_positions.get(&key);
                                    entry.map(|e| if from {
                                        e.1
                                    } else { e.0 })
                                };
                                view! {
                                    <path d={move || {
                                        let Some(start_pos) = get_pos(true) else {
                                            return "".to_owned();
                                        };
                                        let Some(end_pos) = get_pos(false) else {
                                            return "".to_owned();
                                        };
                                        get_path(start_pos, end_pos)
                                    }} stroke="white" fill="transparent" />
                                }
                            } />
                        </svg>
                    </div>
            </div>
        </div>
    }
}

#[component]
fn GraphPipe(
    #[prop(into)] id: String,
    #[prop(into)] node_position: Signal<(f64, f64)>,
    pipe: GraphPipe,
    #[prop(into)] on_input_down: Callback<PointerEvent>,
    #[prop(into)] on_output_down: Callback<PointerEvent>,
) -> impl IntoView {
    let input_ref: NodeRef<Div> = create_node_ref();
    let output_ref: NodeRef<Div> = create_node_ref();
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
    create_effect(move |_| {
        let Some(context) = use_context::<GraphContext>() else {
            return;
        };
        let node_position = node_position.get();
        let get_center = |node_ref: &NodeRef<Div>| {
            node_ref
                .get()
                .map(|f| f.get_bounding_client_rect())
                .map(|r| {
                    (
                        r.x() + r.width() / 2_f64 - node_position.0,
                        r.y() + r.height() / 2_f64 - node_position.1,
                    )
                })
                .unwrap_or_default()
        };
        let input_pos = get_center(&input_ref);
        let output_pos = get_center(&output_ref);
        context.pipe_positions.update(|l| {
            l.insert(
                (id.to_string(), pipe.id.to_string()),
                (input_pos, output_pos),
            );
        });
    });
    view! {
        <div class="row gap-xs align-center justify-between">
            <div _ref={input_ref} on:pointerdown=input_down class={move || format!("m-sm dot {}", if direction.get_value() == PipeDirection::Output { "".to_owned() } else { bg_class.get_value().clone() })} />
            <p>{pipe.name}</p>
            <div _ref={output_ref} on:pointerdown=output_down class={move || format!("m-sm dot {}", if direction.get_value() == PipeDirection::Input { "".to_owned() } else { bg_class.get_value().clone() })} />
        </div>
    }
}
