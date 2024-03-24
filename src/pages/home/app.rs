use leptos::*;
use shared::models::app::VulpineApp;
use strum::{EnumIter, IntoEnumIterator};

use super::{general::GeneralAppView, resources::ResorcesAppView, actions::ActionsAppView};

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum AppTab {
    General,
    Resources,
    Actions,
}

impl AppTab {
    fn display_name(&self) -> &str {
        match self {
            AppTab::General => "General",
            AppTab::Resources => "Resources",
            AppTab::Actions => "Actions",
        }
    }
}

#[component]
pub fn HomeAppView(
    app: RwSignal<VulpineApp>,
    #[prop(into)] edit: MaybeSignal<bool>,
) -> impl IntoView {
    let current_tab = create_rw_signal(AppTab::General);
    
    let view = create_memo(move |_| match current_tab.get() {
        AppTab::General => view! {<GeneralAppView app={app} edit={edit} />}.into_view(),
        AppTab::Resources => view! {<ResorcesAppView app={app} edit={edit} />}.into_view(),
        AppTab::Actions => view! {<ActionsAppView app={app} edit={edit} />}.into_view(),
    });

    view! {
        <div class="col gap-xs flex h-full">
            <div class="row overflow-x gap-xs card paper ph-xs pv-none">
                {move || AppTab::iter().map(|tab| {
                    let cloned_tab = tab.clone();
                    let tab_name = tab.display_name().to_string();
                    let on_click = move |_| current_tab.set(cloned_tab);
                    let current_tab = current_tab.clone();
                    view! {
                        <button class="btn" class:active={move || current_tab.get() == tab}
                            on:click=on_click>
                            {tab_name}
                        </button>
                    }
                }).collect_view()}
            </div>
            {view}
            Value: {move || format!("{:?}", app.get())}
        </div>
    }
}
