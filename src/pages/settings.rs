use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="col gap-md container-md">
            <div class="col">
                <div class="row align-center wrap">
                    <a href="/" class="btn row secondary p-xs hide-sm">
                        <i class="ph-light ph-arrow-left text-icon"/>
                    </a>
                    <h1 class="font-bold ph-sm">"Settings"</h1>
                </div>
                <hr />
            </div>
            <div class="col gap-xs container-xs w-full">
                <h2 class="font-bold">"Personalization"</h2>
                <div class="col gap-xs">
                    <div class="form-group justify-between">
                        <label for="theme">"Theme"</label>
                        <select id="theme">
                            <option>"System"</option>
                            <option>"Light"</option>
                            <option>"Dark"</option>
                        </select>
                    </div>
                    <div class="form-group justify-between">
                        <label for="native-titlebar">"Native titlebar"</label>
                        <input type="checkbox" id="native-titlebar" checked="checked" />
                    </div>
                </div>
            </div>
        </div>
    }
}
