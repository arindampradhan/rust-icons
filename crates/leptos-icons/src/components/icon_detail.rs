use leptos::prelude::*;
use rust_icons_core::snippets::{self, SnippetCategory, SnippetType};
use rust_icons_core::types::ResolvedIcon;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::api;

#[component]
pub fn IconDetail(prefix: String, name: String, on_close: Callback<()>) -> impl IntoView {
    let icon_id = format!("{prefix}:{name}");
    let (icon_data, set_icon_data) = signal(None::<ResolvedIcon>);
    let (svg_html, set_svg_html) = signal(None::<String>);
    let (copied_type, set_copied_type) = signal(None::<String>);
    let (copied_message, set_copied_message) = signal(None::<String>);
    let (hovered_snippet, set_hovered_snippet) = signal(None::<(SnippetType, String)>);

    // Fetch icon data on mount
    {
        let prefix = prefix.clone();
        let name = name.clone();
        spawn_local(async move {
            match api::fetch_icon_data(&prefix, &name).await {
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
    }

    let copy_snippet = move |snippet_type: SnippetType| {
        if let Some(icon) = icon_data.get() {
            let snippet = snippets::generate(&icon, snippet_type);
            let window = web_sys::window().unwrap();
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&snippet);

            let type_name = snippet_type.name().to_string();
            set_copied_type.set(Some(type_name.clone()));
            set_copied_message.set(Some(format!("Copied {}!", type_name)));

            // Clear the button indicator after a short delay
            if let Some(window) = web_sys::window() {
                let closure = Closure::once(move || {
                    set_copied_type.set(None);
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    800,
                );
                closure.forget();
            }

            // Clear the message after 2 seconds
            if let Some(window) = web_sys::window() {
                let closure = Closure::once(move || {
                    set_copied_message.set(None);
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    2000,
                );
                closure.forget();
            }
        }
    };

    let on_snippet_hover = move |snippet_type: SnippetType| {
        if let Some(icon) = icon_data.get() {
            let snippet = snippets::generate(&icon, snippet_type);
            set_hovered_snippet.set(Some((snippet_type, snippet)));
        }
    };

    let on_snippet_leave = move || {
        set_hovered_snippet.set(None);
    };

    let icon_id_display = icon_id.clone();
    let prefix_clone = prefix.clone();

    view! {
        <div class="icon-detail">
            <button class="close-btn" on:click=move |_| on_close.run(())>
                "\u{00d7}"
            </button>

            // Copied message toast
            <Show when=move || copied_message.get().is_some()>
                <div class="copied-toast">
                    {move || copied_message.get().unwrap_or_default()}
                </div>
            </Show>

            // Code preview popup
            <Show when=move || hovered_snippet.get().is_some()>
                <div class="code-preview-popup">
                    {move || {
                        hovered_snippet.get().map(|(snippet_type, code)| {
                            let highlighted = highlight_code(&code, snippet_type);
                            view! {
                                <div class="code-preview-header">
                                    {snippet_type.name()}
                                </div>
                                <pre class="code-preview-content">
                                    <code 
                                        class=format!("language-{}", get_language_class(snippet_type))
                                        inner_html=highlighted
                                    />
                                </pre>
                            }
                        })
                    }}
                </div>
            </Show>

            // Icon preview
            <div class="preview-section">
                <div class="preview-svg" inner_html=move || {
                    svg_html.get().unwrap_or_else(|| "Loading...".to_string())
                } />
                <div class="icon-id">{icon_id_display}</div>
            </div>

            // Rust snippets section
            <div class="detail-section">
                <div class="section-label">"Rust Components"</div>
                <div class="snippet-buttons">
                    {move || {
                        let types = SnippetType::by_category(SnippetCategory::Rust);
                        let copied = copied_type.get();

                        types.into_iter().map(|snippet_type| {
                            let is_copied = copied.as_ref().is_some_and(|c| c == snippet_type.name());
                            let copy_fn = copy_snippet;
                            let hover_fn = on_snippet_hover;
                            let leave_fn = on_snippet_leave;

                            view! {
                                <button
                                    class="snippet-btn"
                                    class:copied=is_copied
                                    on:click=move |_| copy_fn(snippet_type)
                                    on:mouseenter=move |_| hover_fn(snippet_type)
                                    on:mouseleave=move |_| leave_fn()
                                >
                                    {snippet_type.name()}
                                    {snippet_type.tag().map(|tag| view! {
                                        <sup class="tag">{tag}</sup>
                                    })}
                                    {if is_copied {
                                        Some(view! { <span class="copied-indicator">" ✓"</span> })
                                    } else {
                                        None
                                    }}
                                </button>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // Snippets section
            <div class="detail-section">
                <div class="section-label">"Snippets"</div>
                <div class="snippet-buttons">
                    {move || {
                        let types = SnippetType::by_category(SnippetCategory::Snippets);
                        let copied = copied_type.get();

                        types.into_iter().map(|snippet_type| {
                            let is_copied = copied.as_ref().is_some_and(|c| c == snippet_type.name());
                            let copy_fn = copy_snippet;
                            let hover_fn = on_snippet_hover;
                            let leave_fn = on_snippet_leave;

                            view! {
                                <button
                                    class="snippet-btn"
                                    class:copied=is_copied
                                    on:click=move |_| copy_fn(snippet_type)
                                    on:mouseenter=move |_| hover_fn(snippet_type)
                                    on:mouseleave=move |_| leave_fn()
                                >
                                    {snippet_type.name()}
                                    {snippet_type.tag().map(|tag| view! {
                                        <sup class="tag">{tag}</sup>
                                    })}
                                    {if is_copied {
                                        Some(view! { <span class="copied-indicator">" ✓"</span> })
                                    } else {
                                        None
                                    }}
                                </button>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // Download section
            <div class="detail-section">
                <div class="section-label">"Download"</div>
                <div class="download-buttons">
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Svg
                        ext="svg"
                        set_copied_message=set_copied_message
                    />
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Leptos
                        ext="rs"
                        set_copied_message=set_copied_message
                    />
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Yew
                        ext="rs"
                        set_copied_message=set_copied_message
                    />
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Dioxus
                        ext="rs"
                        set_copied_message=set_copied_message
                    />
                </div>
            </div>

            // Links section
            <div class="detail-section">
                <div class="section-label">"Links"</div>
                <div class="external-links">
                    <a
                        class="external-link"
                        href=format!("https://icon-sets.iconify.design/{prefix_clone}/?icon-filter={name}")
                        target="_blank"
                    >
                        "Iconify"
                    </a>
                    <a
                        class="external-link"
                        href=format!("https://uno.antfu.me/?s=i-{prefix}-{name}", prefix=prefix.clone(), name=name.clone())
                        target="_blank"
                    >
                        "UnoCSS"
                    </a>
                </div>
            </div>
        </div>
    }
}

fn get_language_class(snippet_type: SnippetType) -> &'static str {
    match snippet_type {
        SnippetType::Leptos | SnippetType::Yew | SnippetType::Dioxus => "rust",
        SnippetType::Svg | SnippetType::SvgSymbol => "xml",
        SnippetType::DataUrl | SnippetType::Url | SnippetType::Base64 => "text",
        SnippetType::Iconify => "json",
        SnippetType::CssBackground => "css",
        SnippetType::Jsx | SnippetType::React => "jsx",
        SnippetType::ReactTs | SnippetType::Qwik | SnippetType::Solid => "tsx",
        SnippetType::Vue => "vue",
        SnippetType::VueTs => "typescript",
        SnippetType::Svelte => "svelte",
        SnippetType::Astro => "astro",
    }
}

fn highlight_code(code: &str, _snippet_type: SnippetType) -> String {
    // Simple HTML escape for safety
    code
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[component]
fn DownloadButton(
    icon_data: ReadSignal<Option<ResolvedIcon>>,
    snippet_type: SnippetType,
    ext: &'static str,
    set_copied_message: WriteSignal<Option<String>>,
) -> impl IntoView {
    let download = move |_| {
        if let Some(icon) = icon_data.get() {
            let content = snippets::generate(&icon, snippet_type);
            let component_name =
                snippets::to_component_name(&format!("{}:{}", icon.prefix, icon.name));
            let filename = format!("{component_name}.{ext}");

            // Create blob and download
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

            // Show download message
            set_copied_message.set(Some(format!("Downloaded {}!", filename)));

            // Clear the message after 2 seconds
            if let Some(window) = web_sys::window() {
                let closure = Closure::once(move || {
                    set_copied_message.set(None);
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    2000,
                );
                closure.forget();
            }
        }
    };

    view! {
        <button class="download-btn" on:click=download>
            {snippet_type.name()}
        </button>
    }
}
