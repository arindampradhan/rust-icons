use leptos::prelude::*;

#[component]
pub fn SearchBar(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(default = "Search...")] placeholder: &'static str,
) -> impl IntoView {
    view! {
        <div class="search-bar">
            <input
                type="text"
                placeholder=placeholder
                prop:value=move || value.get()
                on:input:target=move |ev| {
                    set_value.set(ev.target().value());
                }
            />
            <Show when=move || !value.get().is_empty()>
                <button
                    class="clear-btn"
                    on:click=move |_| set_value.set(String::new())
                >
                    "\u{00d7}"
                </button>
            </Show>
        </div>
    }
}
