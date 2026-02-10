use leptos::prelude::*;
use rust_icons_core::snippets::{self, SnippetCategory, SnippetType};
use rust_icons_core::types::ResolvedIcon;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::api;

#[component]
pub fn IconDetail(prefix: String, name: String, on_close: Callback<()>) -> impl IntoView {
    let icon_id = format!("{prefix}:{name}");
    let (icon_data, set_icon_data) = signal(None::<ResolvedIcon>);
    let (svg_html, set_svg_html) = signal(None::<String>);
    let (copied_type, set_copied_type) = signal(None::<String>);

    // Fetch icon data on mount or when name changes
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

    let copy_snippet = move |snippet_type: SnippetType| {
        if let Some(icon) = icon_data.get() {
            let snippet = snippets::generate(&icon, snippet_type);
            let window = web_sys::window().unwrap();
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&snippet);

            let type_name = snippet_type.name().to_string();
            set_copied_type.set(Some(type_name.clone()));

            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(2_000).await;
                set_copied_type.set(None);
            });
        }
    };
    
    // Helper for download
    let download = move |snippet_type: SnippetType, ext: &'static str| {
        if let Some(icon) = icon_data.get() {
            let content = snippets::generate(&icon, snippet_type);
            let component_name = snippets::to_component_name(&format!("{}:{}", icon.prefix, icon.name));
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

    view! {
        <div class="drawer-content h-full flex">
            // ── Large Preview (Left) ─────────────────────────
            <div class="drawer-preview">
                <div class="absolute top-4 left-4 font-sans text-xs font-bold bg-white px-2 py-1 border border-black">
                    "PREVIEW"
                </div>
                <div class="scale-[2.0]" inner_html=move || {
                    svg_html.get().unwrap_or_else(|| "Loading...".to_string())
                } />
            </div>

            // ── Details (Right) ──────────────────────────────
            <div class="drawer-details flex flex-col h-full">
                <div class="flex justify-between items-center mb-8 border-b border-gray-200 pb-4">
                    <h3 class="text-3xl font-black font-serif">{name.clone()}</h3>
                    <button 
                        class="p-2 hover:bg-gray-200 rounded-full transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        // Close Icon (X)
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                    </button>
                </div>

                <div class="flex-1 overflow-y-auto space-y-8">
                    // Actions
                    <div>
                        <h4 class="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">"Actions"</h4>
                        <div class="flex gap-4 flex-wrap">
                            <button 
                                class="flex items-center gap-2 bg-black text-white px-4 py-2 font-sans font-bold text-sm hover:bg-[#b91c1c] transition-colors"
                                on:click=move |_| copy_snippet(SnippetType::Svg)
                            >
                                {move || if copied_type.get().as_deref() == Some("SVG") { "Copied!" } else { "Copy SVG" }}
                            </button>
                            <button 
                                class="flex items-center gap-2 border border-black px-4 py-2 font-sans font-bold text-sm hover:bg-gray-50 transition-colors"
                                on:click=move |_| copy_snippet(SnippetType::Jsx)
                            >
                                {move || if copied_type.get().as_deref() == Some("JSX") { "Copied!" } else { "Copy JSX" }}
                            </button>
                            <button 
                                class="flex items-center gap-2 border border-black px-4 py-2 font-sans font-bold text-sm hover:bg-gray-50 transition-colors"
                                on:click=move |_| download(SnippetType::Svg, "svg")
                            >
                                "Download SVG"
                            </button>
                        </div>
                    </div>

                    // Snippets Grid
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div>
                            <h4 class="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">"Component"</h4>
                            <div class="space-y-2">
                                <div 
                                    class="font-mono text-xs bg-gray-50 p-3 border border-gray-200 truncate cursor-pointer hover:border-black"
                                    on:click=move |_| copy_snippet(SnippetType::Leptos)
                                >
                                    {format!("<{} />", snippets::to_component_name(&icon_id))}
                                </div>
                                <div class="font-sans text-xs text-gray-400 mt-1">"Click to copy Leptos component"</div>
                            </div>
                        </div>
                        <div>
                            <h4 class="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">"Import"</h4>
                             <div 
                                class="font-mono text-xs bg-gray-50 p-3 border border-gray-200 truncate cursor-pointer hover:border-black"
                                on:click=move |_| copy_snippet(SnippetType::Leptos) // Usually import is part of the snippet or same action
                            >
                                {format!("use {}::*;", prefix)}
                            </div>
                        </div>
                    </div>

                    // Links
                    <div>
                        <h4 class="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">"Links"</h4>
                        <div class="flex gap-2">
                             <a
                                class="px-3 py-1 border border-gray-300 text-xs font-sans hover:border-black hover:text-black transition-colors"
                                href=format!("https://icon-sets.iconify.design/{}/?icon-filter={}", prefix, name)
                                target="_blank"
                            >
                                "Iconify"
                            </a>
                            <a
                                class="px-3 py-1 border border-gray-300 text-xs font-sans hover:border-black hover:text-black transition-colors"
                                href=format!("https://uno.antfu.me/?s=i-{}-{}", prefix, name)
                                target="_blank"
                            >
                                "UnoCSS"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
