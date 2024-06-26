use super::app::*;
use crate::api::fs::*;
use leptos::*;
use leptos_router::*;
use shared::models::app::VulpineApp;

#[component]
pub fn HomeDetailsView(#[prop(optional_no_strip, into)] id: MaybeProp<String>) -> impl IntoView {
    let navigate = store_value(use_navigate());
    let stored_id = store_value(id);
    let show = move || stored_id.get_value().get().is_none();
    let current_id = create_rw_signal(None);
    let is_editing: Memo<bool> = create_memo(move |_| current_id.get().is_some());
    let fetched = create_local_resource(
        move || stored_id.get_value().get(),
        |id| async {
            let Some(id) = id else {
                return None;
            };
            get_app(id).await
        },
    );
    let app = create_rw_signal(VulpineApp::default());
    create_effect(move |_| {
        app.set(fetched.get().flatten().unwrap_or_default().clone());
    });
    let on_edit = move |_| {
        spawn_local(async move {
            let Some(id) = current_id.get_untracked() else {
                current_id.set(stored_id.get_value().get_untracked());
                return;
            };
            let app = app.get_untracked();
            if update_app(id.clone(), app.clone(), false).await {
                fetched.try_update(|_| Some(app));
                current_id.set(None);
            }
        });
    };
    let on_delete = move |_| {
        spawn_local(async move {
            let Some(id) = stored_id.get_value().get_untracked() else {
                return;
            };
            if delete_app(id).await {
                navigate.get_value()("", Default::default());
            }
        });
    };
    view! {
        <div class="col flex min-w-md h-full overflow-y mp-none gap-sm" class:show-sm={move || stored_id.get_value().get().is_none()}>
            <Show when={move || !show()}>
                <div class="row align-center gap-sm card paper pv-xs ph-md">
                    <a href="/" class="btn secondary row p-xs hide-sm">
                        <i class="ph-light ph-arrow-left text-icon"/>
                    </a>
                    <h2 class="row flex mp-none">
                        <Show when={move || is_editing.get()}>
                            "Edit: "
                        </Show>
                        {move || current_id.get().or(stored_id.get_value().get()).unwrap_or_default()}
                    </h2>
                    <button class="btn primary p-xs" on:click=on_edit>
                        {move || if current_id.get().is_some() {
                            view! {
                                <i class="ph-light ph-floppy-disk text-icon"/>
                            }.into_view()
                        } else {
                            view! {
                                <i class="ph-light ph-pencil-simple text-icon"/>
                            }.into_view()
                        }}
                    </button>
                    <button class="btn secondary p-xs" on:click=on_delete>
                        <i class="ph-light ph-trash text-icon"/>
                    </button>
                </div>
            </Show>
            <div class="flex col justify-center ph-sm card paper">
            {move || match stored_id.get_value().get() {
                Some(_) => view!{
                    <HomeAppView app={app} edit={is_editing} />
                }.into_view(),
                None => view! {<p class="text-center">"Nothing selected"</p>}.into_view()
            }}
            </div>
        </div>
    }
}
