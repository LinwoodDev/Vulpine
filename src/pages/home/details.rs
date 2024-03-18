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
        <div class="col card paper flex min-w-md h-full overflow-y" class:show-sm={move || stored_id.get_value().get().is_none()}>
            <Show when={move || !show()}>
                <div class="row align-center gap-sm card paper pv-xs ph-md">
                    <a href="/" class="btn secondary p-xs hide-sm"><img class="invert icon" title="Home" src="/public/icons/house-light.svg" alt="House icon"/></a>
                    <h2 class="row flex">
                        {move || current_id.get().or(stored_id.get_value().get()).unwrap_or_default()}
                    </h2>
                    <button class="btn primary" on:click=on_edit>
                        {move || if current_id.get().is_some() {
                            view! {
                                <img class="invert icon" title="Save" src="/public/icons/floppy-disk-light.svg" alt="Floppy disk icon"/>
                            }.into_view()
                        } else {
                            view! {
                                <img class="invert icon" title="Edit" src="/public/icons/pencil-simple-light.svg" alt="Pencil icon"/>
                            }.into_view()
                        }}
                    </button>
                    <button class="btn secondary" on:click=on_delete><img class="invert icon" title="Delete" src="/public/icons/trash-light.svg" alt="Trash icon"/></button>
                </div>
            </Show>
            <div class="flex col justify-center ph-sm">
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
