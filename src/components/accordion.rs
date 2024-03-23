use leptos::*;

#[derive(Clone)]
pub struct AccordionContext {
    pub value: Signal<Option<String>>,
    pub on_change: Option<Callback<Option<String>>>,
}

#[component]
pub fn Accordion(
    children: Children,
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<String>>>,
) -> impl IntoView {
    let context = AccordionContext { value, on_change };
    provide_context(context);
    view! {
        <div class="col align-stretch gap-xs">
            {children()}
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct AccordionItemContext {
    pub value: Signal<bool>,
    pub on_change: Callback<()>,
}

#[component]
pub fn AccordionItem(#[prop(into)] key: String, children: Children) -> impl IntoView {
    let key = store_value(key);
    let context = use_context::<AccordionContext>();
    let on_change = store_value(context.as_ref().map(|context| context.on_change).flatten());
    let is_active = create_memo(move |_| {
        context
            .as_ref()
            .and_then(|context| context.value.get())
            .map_or(false, |e| key.get_value() == e))
    });
    let on_click = move |_| {
        on_change.get_value().map(|on_change| {
            on_change.call(if is_active.get_untracked() {
                None
            } else {
                Some(key.get_value())
            })
        });
    };
    let item_context = AccordionItemContext {
        value: is_active.into(),
        on_change: Callback::new(on_click),
    };
    provide_context(item_context);
    view! {
        <div class="card paper col gap-xs p-sm">
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItemTitle(
    #[prop(into)] title: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = use_context::<AccordionItemContext>();
    let is_active = create_memo(move |_| {
        context
            .as_ref()
            .map_or(false, |context| context.value.get())
    });
    let children = store_value(children);
    let on_change = store_value(context.map(|context| context.on_change));
    view! {
        <div class="row gap-xs align-center">
            <a class="bold text no-decoration pv-sm flex" class:primary=is_active href="#" on:click={move |e| {
                e.prevent_default();
                on_change.get_value().map(|on_change| on_change.call(()));
            }}>
                {title}
            </a>
            <Show when={move || children.get_value().is_some()}>
                {children.with_value(|children| children.clone().map(|children| children()))}
            </Show>
        </div>
    }
}

#[component]
pub fn AccordionItemContent(children: ChildrenFn) -> impl IntoView {
    let context = use_context::<AccordionItemContext>();
    let is_active = create_memo(move |_| {
        context
            .as_ref()
            .map_or(false, |context| context.value.get())
    });
    view! {
        <Show when={move || is_active.get()}>
            {children()}
        </Show>
    }
}
