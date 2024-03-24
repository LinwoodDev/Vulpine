use leptos::*;
use shared::models::app::VulpineAction;

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
                <div class="flex view card paper">
                </div>
            </div>
        </Show>
    }
}
