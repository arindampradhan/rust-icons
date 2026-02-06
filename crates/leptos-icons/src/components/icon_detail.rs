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
    let (active_tab, set_active_tab) = signal(SnippetCategory::Rust);

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

            spawn_local(async move {
                gloo_net::http::Request::get("data:,").send().await.ok();
                set_copied_type.set(None);
            });
        }
    };

    let icon_id_display = icon_id.clone();
    let prefix_clone = prefix.clone();

    view! {
        <div class="icon-detail">
            <button class="close-btn" on:click=move |_| on_close.run(())>
                "\u{00d7}"
            </button>

            // Icon preview
            <div class="preview-section">
                <div class="preview-svg" inner_html=move || {
                    svg_html.get().unwrap_or_else(|| "Loading...".to_string())
                } />
                <div class="icon-id">{icon_id_display}</div>
            </div>

            // Category tabs
            <div class="snippet-tabs">
                <SnippetTab
                    category=SnippetCategory::Rust
                    label="Rust"
                    active_tab=active_tab
                    set_active_tab=set_active_tab
                />
                <SnippetTab
                    category=SnippetCategory::Snippets
                    label="Snippets"
                    active_tab=active_tab
                    set_active_tab=set_active_tab
                />
                <SnippetTab
                    category=SnippetCategory::Components
                    label="Components"
                    active_tab=active_tab
                    set_active_tab=set_active_tab
                />
                <SnippetTab
                    category=SnippetCategory::Links
                    label="Links"
                    active_tab=active_tab
                    set_active_tab=set_active_tab
                />
            </div>

            // Snippet buttons
            <div class="snippet-buttons">
                {move || {
                    let tab = active_tab.get();
                    let types = SnippetType::by_category(tab);
                    let copied = copied_type.get();

                    types.into_iter().map(|snippet_type| {
                        let is_copied = copied.as_ref().map(|c| c == snippet_type.name()).unwrap_or(false);
                        let copy_fn = copy_snippet;

                        view! {
                            <button
                                class="snippet-btn"
                                class:copied=is_copied
                                on:click=move |_| copy_fn(snippet_type)
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

            // Download section
            <div class="download-section">
                <div class="section-label">"Download"</div>
                <div class="download-buttons">
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Svg
                        ext="svg"
                    />
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Leptos
                        ext="rs"
                    />
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Yew
                        ext="rs"
                    />
                    <DownloadButton
                        icon_data=icon_data
                        snippet_type=SnippetType::Dioxus
                        ext="rs"
                    />
                </div>
            </div>

            // View on external sites
            <div class="external-links">
                <div class="section-label">"View on"</div>
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

            // Install iconset
            <InstallIconset prefix=prefix.clone() />
        </div>
    }
}

#[component]
fn SnippetTab(
    category: SnippetCategory,
    label: &'static str,
    active_tab: ReadSignal<SnippetCategory>,
    set_active_tab: WriteSignal<SnippetCategory>,
) -> impl IntoView {
    view! {
        <button
            class="tab-btn"
            class:active=move || active_tab.get() == category
            on:click=move |_| set_active_tab.set(category)
        >
            {label}
        </button>
    }
}

#[component]
fn DownloadButton(
    icon_data: ReadSignal<Option<ResolvedIcon>>,
    snippet_type: SnippetType,
    ext: &'static str,
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
        }
    };

    view! {
        <button class="download-btn" on:click=download>
            {snippet_type.name()}
        </button>
    }
}

#[component]
fn InstallIconset(prefix: String) -> impl IntoView {
    let (selected_pm, set_selected_pm) = signal("pnpm");
    let (copied, set_copied) = signal(false);

    let package_managers = [
        ("pnpm", "pnpm add -D"),
        ("npm", "npm i -D"),
        ("yarn", "yarn add -D"),
        ("bun", "bun add -D"),
    ];

    let copy_command = {
        let prefix = prefix.clone();
        move |_| {
            let pm = selected_pm.get();
            let cmd = match pm {
                "npm" => "npm i -D",
                "yarn" => "yarn add -D",
                "bun" => "bun add -D",
                _ => "pnpm add -D",
            };
            let command = format!("{cmd} @iconify-json/{prefix}");

            let window = web_sys::window().unwrap();
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&command);

            set_copied.set(true);
            spawn_local(async move {
                gloo_net::http::Request::get("data:,").send().await.ok();
                set_copied.set(false);
            });
        }
    };

    view! {
        <div class="install-section">
            <a
                class="install-header"
                href="https://iconify.design/docs/icons/json.html"
                target="_blank"
            >
                "Install Iconify Iconset"
            </a>

            <div class="pm-selector">
                {package_managers.iter().map(|(pm, _)| {
                    let pm = *pm;
                    view! {
                        <button
                            class="pm-btn"
                            class:active=move || selected_pm.get() == pm
                            on:click=move |_| set_selected_pm.set(pm)
                        >
                            {pm}
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="install-command">
                <code>
                    <span class="cmd-pm">{move || selected_pm.get()}</span>
                    <span class="cmd-action">{move || {
                        match selected_pm.get() {
                            "npm" => " i -D ",
                            _ => " add -D ",
                        }
                    }}</span>
                    <span class="cmd-package">"@iconify-json/"{prefix.clone()}</span>
                </code>
                <button class="copy-cmd-btn" on:click=copy_command>
                    {move || if copied.get() { "✓" } else { "Copy" }}
                </button>
            </div>
        </div>
    }
}
