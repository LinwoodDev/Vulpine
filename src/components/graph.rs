use leptos::{logging::log, *};
use web_sys::MouseEvent;

#[derive(Debug, Clone, Copy)]
pub struct GraphNode {
    pub x: i32,
    pub y: i32,
    pub id: usize,
}

#[component]
pub fn GraphView<FN, N>(
    #[prop(into)] nodes: MaybeSignal<Vec<GraphNode>>,
    build_node: FN,
) -> impl IntoView
where
    FN: Fn(&GraphNode) -> N + 'static,
    N: IntoView + 'static,
{
    let current_position = create_rw_signal((0, 0));
    let dragging_start = create_rw_signal::<Option<(i32, i32)>>(None);
    let on_down = move |e: MouseEvent| {
        e.prevent_default();
        let last = current_position.get_untracked();
        dragging_start.set(Some((e.offset_x() - last.0, e.offset_y() - last.1)));
    };
    let on_up = move |e: MouseEvent| {
        e.prevent_default();
        dragging_start.set(None);
    };
    let on_move = move |e: MouseEvent| {
        let start = dragging_start.get_untracked();
        let Some((start_x, start_y)) = start else {
            return;
        };
        e.prevent_default();
        let offset_x = e.offset_x();
        let offset_y = e.offset_y();
        current_position.update(|last| {
            last.0 = offset_x - start_x;
            last.1 = offset_y - start_y;
            log!("Current position: {:?}", last);
        });
    };
    view! {
        <div class="w-full h-full view no-overflow" on:mousedown=on_down on:mouseup=on_up on:mousemove=on_move>
            <svg width="100%" height="100%">
                <pattern id="background-pattern" x={move || current_position.get().0} y={move || current_position.get().1} width="32" height="32" patternUnits="userSpaceOnUse">
                    <circle cx="14" cy="14" r="4" fill="white" />
                </pattern>
                <rect width="100%" height="100%" fill="url(#background-pattern)" />
                <circle cx={move || current_position.get().0} cy={move || current_position.get().1} r="4" fill="red" />
            </svg>
            <div style={move || {
                let pos = current_position.get();
                format!("transform: translate({}px, {}px);", pos.0, pos.1)
            }} class="w-full h-full">
                <For each={move || nodes.get()} key={|e|e.id} children={move |node| {
                    view! {
                        <div class="card paper" style={format!("position: absolute; left: {}px; top: {}px;", node.x, node.y)}>
                            {build_node(&node)}
                        </div>
                    }
                }} />
            </div>
        </div>
    }
}
