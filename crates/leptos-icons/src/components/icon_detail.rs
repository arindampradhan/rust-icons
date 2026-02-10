use leptos::prelude::*;
use rust_icons_core::snippets::{self, SnippetType};
use rust_icons_core::types::ResolvedIcon;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::api;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Rust,
    Snippets,
    Links,
}

#[component]
pub fn IconDetail(prefix: String, name: String, on_close: Callback<()>) -> impl IntoView {
    let icon_id = format!("{prefix}:{name}");
    let (icon_data, set_icon_data) = signal(None::<ResolvedIcon>);
    let (svg_html, set_svg_html) = signal(None::<String>);
    let (copied_label, set_copied_label) = signal(None::<String>);
    let (active_tab, set_active_tab) = signal(Tab::Rust);

    // Fetch icon data
    let prefix_clone = prefix.clone();
    let name_clone = name.clone();
    Effect::new(move || {
        let p = prefix_clone.clone();
        let n = name_clone.clone();
        spawn_local(async move {
            match api::fetch_icon_data(&p, &n).await {
                Ok(icon) => {
                    let svg = rust_icons_core::svg::build_svg(&icon);
                    set_svg_html.set(Some(svg));
                    set_icon_data.set(Some(icon));
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to fetch icon: {e}").into());
                }
            }
        });
    });

    let copy_snippet = move |snippet_type: SnippetType, label: &'static str| {
        if let Some(icon) = icon_data.get() {
            let snippet = snippets::generate(&icon, snippet_type);
            let window = web_sys::window().unwrap();
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&snippet);

            set_copied_label.set(Some(label.to_string()));
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(2_000).await;
                set_copied_label.set(None);
            });
        }
    };

    let download = move |snippet_type: SnippetType, ext: &'static str| {
        if let Some(icon) = icon_data.get() {
            let content = snippets::generate(&icon, snippet_type);
            let component_name =
                snippets::to_component_name(&format!("{}:{}", icon.prefix, icon.name));
            let filename = format!("{component_name}.{ext}");

            let bag = web_sys::BlobPropertyBag::new();
            bag.set_type("text/plain");
            let blob = web_sys::Blob::new_with_str_sequence_and_options(
                &js_sys::Array::of1(&content.into()),
                &bag,
            )
            .unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let document = web_sys::window().unwrap().document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &filename).unwrap();
            a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
            web_sys::Url::revoke_object_url(&url).unwrap();
        }
    };

    let prefix_for_links = prefix.clone();
    let name_for_links = name.clone();

    view! {
        <div class="drawer-content">
            // ── Large Preview (Left) ─────────────────────────
            <div class="drawer-preview">
                <div class="drawer-preview-label">"PREVIEW"</div>
                <div class="drawer-preview-svg" inner_html=move || {
                    svg_html.get().unwrap_or_else(|| "Loading...".to_string())
                } />
            </div>

            // ── Details (Right) ──────────────────────────────
            <div class="drawer-details">
                <div class="drawer-detail-header">
                    <h3 class="drawer-icon-name">{icon_id}</h3>
                    <button class="drawer-close-btn" on:click=move |_| on_close.run(())>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                    </button>
                </div>

                // ── Tab Bar ──────────────────────────────────
                <div class="drawer-tabs">
                    <button
                        class=move || if active_tab.get() == Tab::Rust { "drawer-tab active" } else { "drawer-tab" }
                        on:click=move |_| set_active_tab.set(Tab::Rust)
                    >"Rust"</button>
                    <button
                        class=move || if active_tab.get() == Tab::Snippets { "drawer-tab active" } else { "drawer-tab" }
                        on:click=move |_| set_active_tab.set(Tab::Snippets)
                    >"Snippets"</button>
                    <button
                        class=move || if active_tab.get() == Tab::Links { "drawer-tab active" } else { "drawer-tab" }
                        on:click=move |_| set_active_tab.set(Tab::Links)
                    >"Links"</button>
                </div>

                // ── Tab Content (sub-options) ────────────────
                <div class="drawer-tab-content">
                    {move || match active_tab.get() {
                        Tab::Rust => view! {
                            <div class="drawer-pills">
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Leptos, "Leptos")>
                                    {move || if copied_label.get().as_deref() == Some("Leptos") { "Copied!" } else { "Leptos" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Yew, "Yew")>
                                    {move || if copied_label.get().as_deref() == Some("Yew") { "Copied!" } else { "Yew" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Dioxus, "Dioxus")>
                                    {move || if copied_label.get().as_deref() == Some("Dioxus") { "Copied!" } else { "Dioxus" }}
                                </button>
                            </div>
                        }.into_any(),
                        Tab::Snippets => view! {
                            <div class="drawer-pills">
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Svg, "SVG")>
                                    {move || if copied_label.get().as_deref() == Some("SVG") { "Copied!" } else { "SVG" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::SvgSymbol, "SVG Symbol")>
                                    {move || if copied_label.get().as_deref() == Some("SVG Symbol") { "Copied!" } else { "SVG Symbol" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Iconify, "Iconify")>
                                    {move || if copied_label.get().as_deref() == Some("Iconify") { "Copied!" } else { "Iconify" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Jsx, "JSX")>
                                    {move || if copied_label.get().as_deref() == Some("JSX") { "Copied!" } else { "JSX" }}
                                </button>
                            </div>
                        }.into_any(),
                        Tab::Links => view! {
                            <div class="drawer-pills">
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Url, "URL")>
                                    {move || if copied_label.get().as_deref() == Some("URL") { "Copied!" } else { "URL" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::DataUrl, "Data URL")>
                                    {move || if copied_label.get().as_deref() == Some("Data URL") { "Copied!" } else { "Data URL" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::Base64, "Base64")>
                                    {move || if copied_label.get().as_deref() == Some("Base64") { "Copied!" } else { "Base64" }}
                                </button>
                                <button class="drawer-pill" on:click=move |_| copy_snippet(SnippetType::CssBackground, "CSS")>
                                    {move || if copied_label.get().as_deref() == Some("CSS") { "Copied!" } else { "CSS" }}
                                </button>
                            </div>
                        }.into_any(),
                    }}
                </div>

                // ── Download (persistent) ────────────────────
                <div class="drawer-persistent-section">
                    <h4 class="drawer-section-title">"Download"</h4>
                    <div class="drawer-pills">
                        <button class="drawer-pill" on:click=move |_| download(SnippetType::Svg, "svg")>"SVG"</button>
                        <button class="drawer-pill" on:click=move |_| download(SnippetType::Leptos, "rs")>"Leptos"</button>
                        <button class="drawer-pill" on:click=move |_| download(SnippetType::Yew, "rs")>"Yew"</button>
                        <button class="drawer-pill" on:click=move |_| download(SnippetType::Dioxus, "rs")>"Dioxus"</button>
                    </div>
                </div>

                // ── View on (persistent) ─────────────────────
                <div class="drawer-persistent-section">
                    <h4 class="drawer-section-title">"View on"</h4>
                    <div class="drawer-pills">
                        <a
                            class="drawer-pill"
                            href=format!("https://icon-sets.iconify.design/{}/?icon-filter={}", prefix_for_links, name_for_links)
                            target="_blank"
                        >"Iconify"</a>
                        <a
                            class="drawer-pill"
                            href=format!("https://uno.antfu.me/?s=i-{}-{}", prefix_for_links, name_for_links)
                            target="_blank"
                        >"UnoCSS"</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
