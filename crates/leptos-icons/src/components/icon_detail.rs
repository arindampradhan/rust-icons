use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api;

#[component]
pub fn IconDetail(prefix: String, name: String, on_close: Callback<()>) -> impl IntoView {
    let icon_id = format!("{prefix}:{name}");
    let (svg_html, set_svg_html) = signal(None::<String>);
    let (copied, set_copied) = signal(false);

    // Fetch the full SVG on mount
    {
        let prefix = prefix.clone();
        let name = name.clone();
        spawn_local(async move {
            match api::fetch_svg(&prefix, &name).await {
                Ok(svg) => set_svg_html.set(Some(svg)),
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to fetch SVG: {e}").into());
                }
            }
        });
    }

    let copy_svg = move |_| {
        if let Some(svg) = svg_html.get() {
            let window = web_sys::window().unwrap();
            let clipboard = window.navigator().clipboard();
            let promise = clipboard.write_text(&svg);
            drop(wasm_bindgen_futures::JsFuture::from(promise));
            set_copied.set(true);

            // Reset copied state after 2 seconds
            spawn_local(async move {
                gloo_net::http::Request::get("data:,").send().await.ok();
                // Use a simple timeout via promise
                set_copied.set(false);
            });
        }
    };

    let icon_id_display = icon_id.clone();

    view! {
        <div class="icon-detail">
            <button class="close-btn" on:click=move |_| on_close.run(())>
                "\u{00d7}"
            </button>
            <div class="preview-svg" inner_html=move || {
                svg_html.get().unwrap_or_else(|| "Loading...".to_string())
            } />
            <div class="icon-id">{icon_id_display}</div>
            <div class="actions">
                <button on:click=copy_svg>
                    {move || if copied.get() { "Copied!" } else { "Copy SVG" }}
                </button>
            </div>
        </div>
    }
}
