use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rust_icons_core::search::search_icons;
use rust_icons_core::svg::iconify_img_url;

use crate::api;
use crate::components::icon_detail::IconDetail;
use crate::components::search_bar::SearchBar;
use crate::components::theme_toggle::ThemeToggle;

#[component]
pub fn CollectionPage() -> impl IntoView {
    let params = use_params_map();
    let initial_id = params.read_untracked().get("id").unwrap_or_default();
    let id = move || params.read().get("id").unwrap_or_default();

    let icons_resource = LocalResource::new(move || {
        let prefix = id();
        async move { api::fetch_collection_icons(&prefix).await }
    });

    // Sidebar title: shows full name once loaded, prefix as fallback
    let (sidebar_name, set_sidebar_name) = signal(initial_id.clone());

    let (search, set_search) = signal(String::new());
    let (selected_icon, set_selected_icon) = signal(None::<String>);
    let (selected_category, set_selected_category) = signal(None::<String>);
    let (sidebar_categories, set_sidebar_categories) = signal(Vec::<(String, usize)>::new());
    let (total_icons_count, set_total_icons_count) = signal(0usize);

    view! {
        <div class="collection-layout">
            // ── Sidebar ──────────────────────────────────────
            <aside class="sidebar">
                <div class="sidebar-header">
                    <a href="/" class="sidebar-back-btn" title="Back to Home">
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="19" y1="12" x2="5" y2="12"></line>
                            <polyline points="12 19 5 12 12 5"></polyline>
                        </svg>
                    </a>
                    <h1 class="sidebar-title">{move || sidebar_name.get()}</h1>
                </div>

                <div class="sidebar-search">
                    <div class="sidebar-search-inner">
                        <div class="sidebar-search-icon">
                           <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                        </div>
                        <input
                           type="text"
                           placeholder="Search collections..."
                           class="sidebar-search-input"
                        />
                    </div>
                </div>

                <div class="sidebar-nav">
                    <button
                        class=move || if selected_category.get().is_none() { "sidebar-link active" } else { "sidebar-link" }
                        on:click=move |_| set_selected_category.set(None)
                    >
                        <span>"All"</span>
                        <span class="sidebar-link-count">{move || total_icons_count.get()}</span>
                    </button>
                    <For
                        each=move || sidebar_categories.get()
                        key=|(name, _)| name.clone()
                        let:cat
                    >
                        {
                            let cat_name = cat.0.clone();
                            let cat_name_click = cat.0.clone();
                            let cat_name_class = cat.0.clone();
                            let count = cat.1;
                            view! {
                                <button
                                    class=move || {
                                        if selected_category.get().as_deref() == Some(&cat_name_class) {
                                            "sidebar-link active"
                                        } else {
                                            "sidebar-link"
                                        }
                                    }
                                    on:click=move |_| set_selected_category.set(Some(cat_name_click.clone()))
                                >
                                    <span>{cat_name}</span>
                                    <span class="sidebar-link-count">{count}</span>
                                </button>
                            }
                        }
                    </For>
                </div>

                <div class="sidebar-footer">
                    <div class="sidebar-actions">
                        // Light/Dark mode
                        <ThemeToggle />
                        // Settings
                        <button class="action-btn" title="Settings">
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
                        </button>
                        // Favorites
                        <button class="action-btn" title="Favorites">
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
                        </button>
                        // Menu
                        <button class="action-btn" title="Menu">
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>
                        </button>
                    </div>
                </div>
            </aside>

            // ── Main Content ─────────────────────────────────
            <main class="main-content">
                <Suspense fallback=|| view! { <div class="loading">"Loading collection..."</div> }>
                    {move || {
                        let prefix = id();
                        let prefix_cloned = prefix.clone();
                        Suspend::new(async move {
                            match icons_resource.await {
                            Ok(resp) => {
                                let icon_names = resp.all_icon_names();
                                let total = icon_names.len();
                                let all_icons = icon_names.clone();
                                let prefix_for_for_clone = prefix_cloned.clone();

                                // Extract collection metadata
                                let display_name = resp.info.as_ref()
                                    .map(|i| i.name.clone())
                                    .or(resp.title.clone())
                                    .unwrap_or_else(|| prefix.clone());
                                set_sidebar_name.set(display_name.clone());
                                let author_name = resp.info.as_ref()
                                    .and_then(|i| i.author.as_ref())
                                    .map(|a| format!("@{}", a.name));
                                let license_spdx = resp.info.as_ref()
                                    .and_then(|i| i.license.as_ref())
                                    .map(|l| l.spdx.clone().unwrap_or_else(|| l.title.clone()));

                                let meta_parts: Vec<String> = [
                                    author_name,
                                    license_spdx,
                                    Some(format!("{total} icons")),
                                ].into_iter().flatten().collect();
                                let meta_line = meta_parts.join(" · ");

                                // Populate sidebar categories
                                set_total_icons_count.set(total);
                                let mut cats: Vec<(String, usize)> = resp.categories.iter()
                                    .map(|(name, icons)| (name.clone(), icons.len()))
                                    .collect();
                                cats.sort_by(|a, b| a.0.cmp(&b.0));
                                if !resp.uncategorized.is_empty() {
                                    cats.push(("Uncategorized".to_string(), resp.uncategorized.len()));
                                }
                                set_sidebar_categories.set(cats);

                                // Clone categories map for filtering - these are stable data
                                let categories_map = resp.categories.clone();
                                let uncategorized = resp.uncategorized.clone();

                                // Reactive search + category filter - computed value that reacts to search and category changes
                                let filtered_icons = Signal::derive(move || {
                                    let search_query = search.get();
                                    let search_results = search_icons(&all_icons, &search_query);
                                    
                                    match selected_category.get() {
                                        None => search_results,
                                        Some(cat) => {
                                            if cat == "Uncategorized" {
                                                search_results.into_iter()
                                                    .filter(|name| uncategorized.contains(name))
                                                    .collect()
                                            } else if let Some(cat_icons) = categories_map.get(&cat) {
                                                search_results.into_iter()
                                                    .filter(|name| cat_icons.contains(name))
                                                    .collect()
                                            } else {
                                                search_results
                                            }
                                        }
                                    }
                                });

                                // Group icons by first letter - computed from filtered icons
                                let grouped_icons = Signal::derive(move || {
                                    let icons = filtered_icons.get();
                                    
                                    // Return empty if no icons match
                                    if icons.is_empty() {
                                        return Vec::new();
                                    }
                                    
                                    let mut groups: Vec<(String, Vec<String>)> = Vec::new();

                                    for icon in icons {
                                        let first_char = icon.chars().next().unwrap_or('?');
                                        let group_key = if first_char.is_ascii_digit() {
                                            "#".to_string()
                                        } else {
                                            first_char.to_uppercase().to_string()
                                        };
                                        if let Some(group) = groups.iter_mut().find(|(key, _)| key == &group_key) {
                                            group.1.push(icon);
                                        } else {
                                            groups.push((group_key, vec![icon]));
                                        }
                                    }
                                    groups.sort_by(|a, b| a.0.cmp(&b.0));
                                    groups
                                });

                                let prefix_for_drawer = prefix_cloned.clone();

                                view! {
                                    <header class="collection-detail-header">
                                        <div class="collection-title-group">
                                            <h2 class="collection-title">{display_name}</h2>
                                            <div class="collection-subtitle">
                                                {meta_line}
                                            </div>
                                        </div>

                                        <div class="collection-search-wrapper">
                                            <SearchBar
                                                value=search
                                                set_value=set_search
                                                placeholder="Search icons..."
                                            />
                                        </div>
                                    </header>

                                    <Show when=move || !sidebar_categories.get().is_empty()>
                                        <div class="variant-chips">
                                            <button
                                                class=move || if selected_category.get().is_none() { "variant-chip active" } else { "variant-chip" }
                                                on:click=move |_| set_selected_category.set(None)
                                            >"All"</button>
                                            <For
                                                each=move || sidebar_categories.get()
                                                key=|(name, _)| name.clone()
                                                let:cat
                                            >
                                                {
                                                    let cat_name = cat.0.clone();
                                                    let cat_click = cat.0.clone();
                                                    let cat_class = cat.0.clone();
                                                    view! {
                                                        <button
                                                            class=move || {
                                                                if selected_category.get().as_deref() == Some(&cat_class) {
                                                                    "variant-chip active"
                                                                } else {
                                                                    "variant-chip"
                                                                }
                                                            }
                                                            on:click=move |_| set_selected_category.set(Some(cat_click.clone()))
                                                        >{cat_name}</button>
                                                    }
                                                }
                                            </For>
                                        </div>
                                    </Show>

                                    <div class="icons-grid-section">
                                        {move || {
                                            let groups = grouped_icons.get();
                                            if groups.is_empty() {
                                                let search_query = search.get();
                                                if !search_query.is_empty() {
                                                    view! {
                                                        <div class="empty-state">
                                                            <p>"No icons found matching \"" {search_query} "\""</p>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }
                                            } else {
                                                let prefix_for_group = prefix_for_for_clone.clone();
                                                view! {
                                                    <For
                                                        each=move || grouped_icons.get()
                                                        key=|(letter, _)| letter.clone()
                                                        let:group
                                                    >
                                                        {
                                                            let prefix_for_icons = prefix_for_group.clone();
                                                            view! {
                                                                <div class="letter-group">
                                                                    <div class="letter-header">
                                                                        <h3 class="letter-title">{group.0.clone()}</h3>
                                                                        <div class="letter-separator"></div>
                                                                        <span class="letter-count">
                                                                            {format!("{} ICONS", group.1.len())}
                                                                        </span>
                                                                    </div>

                                                                    <div class="icons-grid">
                                                                        {
                                                                            let prefix_for_inner = prefix_for_icons.clone();
                                                                            view! {
                                                                                <For
                                                                                    each=move || group.1.clone()
                                                                                    key=|name| name.clone()
                                                                                    let:icon_name
                                                                                >
                                                                                    {
                                                                                        let name_clone = icon_name.clone();
                                                                                        let p = prefix_for_inner.clone();
                                                                                        let img_url = iconify_img_url(&p, &icon_name);

                                                                                        view! {
                                                                                            <button
                                                                                                class="icon-item"
                                                                                                on:click=move |_| set_selected_icon.set(Some(name_clone.clone()))
                                                                                            >
                                                                                                <div class="icon-preview-box">
                                                                                                    <img src=img_url alt=icon_name.clone() loading="lazy" width="32" height="32" />
                                                                                                </div>
                                                                                                <div class="icon-info">
                                                                                                    <div class="icon-name" title=icon_name.clone()>{icon_name.clone()}</div>
                                                                                                    <div class="icon-meta">"SVG"</div>
                                                                                                </div>
                                                                                            </button>
                                                                                        }
                                                                                    }
                                                                                </For>
                                                                            }
                                                                        }
                                                                    </div>
                                                                </div>
                                                            }
                                                        }
                                                    </For>
                                                }.into_any()
                                            }
                                        }}
                                    </div>

                                    // ── Drawer ───────────────────────────────────────
                                    {move || {
                                        let is_open = selected_icon.get().is_some();
                                        let current_icon = selected_icon.get().unwrap_or_default();
                                        let p = prefix_for_drawer.clone();

                                        view! {
                                            <div
                                                class=format!("drawer-overlay {}", if is_open { "open" } else { "" })
                                                on:click=move |_| set_selected_icon.set(None)
                                            />
                                            <div class=format!("drawer {}", if is_open { "open" } else { "" })>
                                                <Show when=move || is_open>
                                                    <IconDetail
                                                        prefix=p.clone()
                                                        name=current_icon.clone()
                                                        on_close=Callback::new(move |()| set_selected_icon.set(None))
                                                    />
                                                </Show>
                                            </div>
                                        }
                                    }}
                                }.into_any()
                            }
                            Err(e) => view! {
                                <div class="empty-state">{format!("Failed to load collection: {e}")}</div>
                            }.into_any()
                        }
                        })
                    }}
                </Suspense>
            </main>
        </div>
    }
}
