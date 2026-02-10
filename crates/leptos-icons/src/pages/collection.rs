use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rust_icons_core::search::search_icons;
use rust_icons_core::svg::iconify_img_url;

use crate::api;
use crate::components::icon_detail::IconDetail;
use crate::components::search_bar::SearchBar;

#[component]
pub fn CollectionPage() -> impl IntoView {
    let params = use_params_map();
    let initial_id = params.read_untracked().get("id").unwrap_or_default();
    let id = move || params.read().get("id").unwrap_or_default();

    let icons_resource = LocalResource::new(move || {
        let prefix = id();
        async move { api::fetch_collection_icons(&prefix).await }
    });

    let (search, set_search) = signal(String::new());
    let (selected_icon, set_selected_icon) = signal(None::<String>);

    view! {
        <div class="collection-layout">
            // ── Sidebar ──────────────────────────────────────
            <aside class="sidebar">
                <div class="sidebar-header">
                    <a href="/" class="p-2 hover:bg-black hover:text-white rounded-full transition-colors flex items-center justify-center" title="Back to Home">
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="19" y1="12" x2="5" y2="12"></line>
                            <polyline points="12 19 5 12 12 5"></polyline>
                        </svg>
                    </a>
                    <h1 class="font-serif font-bold text-xl truncate">{initial_id.clone()}</h1>
                </div>
                
                <div class="p-4 border-b border-black bg-white">
                    <div class="relative">
                        <div class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">
                           <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                        </div>
                        <input 
                           type="text" 
                           placeholder="Search collections..." 
                           class="w-full bg-gray-50 border border-gray-200 py-2 pl-9 pr-2 text-sm focus:outline-none focus:border-black font-sans"
                        />
                    </div>
                </div>

                <div class="sidebar-nav">
                    <div class="sidebar-link font-bold bg-black text-white">"All Icons"</div>
                    <div class="sidebar-link">"Material Symbols"</div>
                    <div class="sidebar-link">"Google Material"</div>
                    <div class="sidebar-link">"Carbon"</div>
                    <div class="sidebar-link">"Phosphor"</div>
                    <div class="sidebar-link">"Remix"</div>
                </div>

                <div class="p-4 border-t border-black text-xs font-sans text-center text-gray-500">
                    "© 2026 Icônes"
                </div>
            </aside>

            // ── Main Content ─────────────────────────────────
            <main class="main-content">
                <Suspense fallback=|| view! { <div class="loading">"Loading collection..."</div> }>
                    {move || Suspend::new(async move {
                        let prefix = id();
                        match icons_resource.await {
                            Ok(resp) => {
                                let icon_names = resp.all_icon_names();
                                let total = icon_names.len();
                                let all_icons = icon_names.clone();

                                // Reactive search
                                let filtered_icons = Signal::derive(move || {
                                    search_icons(&all_icons, &search.get())
                                });

                                // Group icons by first letter
                                let grouped_icons = Signal::derive(move || {
                                    let icons = filtered_icons.get();
                                    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
                                    
                                    for icon in icons {
                                        let first_char = icon.chars().next().unwrap_or('?').to_uppercase().to_string();
                                        if let Some(group) = groups.iter_mut().find(|(key, _)| key == &first_char) {
                                            group.1.push(icon);
                                        } else {
                                            groups.push((first_char, vec![icon]));
                                        }
                                    }
                                    groups.sort_by(|a, b| a.0.cmp(&b.0));
                                    groups
                                });

                                let prefix_for_grid = prefix.clone();
                                let prefix_for_drawer = prefix.clone();

                                view! {
                                    <header class="collection-detail-header">
                                        <div class="flex flex-col">
                                            <div class="flex items-center gap-2">
                                                <h2 class="text-2xl font-black font-serif">{prefix.clone()}</h2>
                                            </div>
                                            <div class="text-xs text-gray-500 font-sans mt-1">
                                                {format!("{} icons", total)}
                                            </div>
                                        </div>

                                        <div class="flex items-center gap-4">
                                            <div class="relative w-64">
                                                <SearchBar
                                                    value=search
                                                    set_value=set_search
                                                    placeholder="Search icons..."
                                                />
                                            </div>
                                        </div>
                                    </header>

                                    <div class="variant-chips">
                                        <button class="variant-chip">"Outlined"</button>
                                        <button class="variant-chip">"Filled"</button>
                                        <button class="variant-chip">"Rounded"</button>
                                        <button class="variant-chip">"Sharp"</button>
                                        <button class="variant-chip">"Two Tone"</button>
                                    </div>

                                    <div class="icons-grid-section">
                                        <For
                                            each=move || grouped_icons.get()
                                            key=|(letter, _)| letter.clone()
                                            let:group
                                        >
                                            <div class="letter-group">
                                                <div class="letter-header">
                                                    <h3 class="letter-title">{group.0.clone()}</h3>
                                                    <div class="h-[1px] flex-1 bg-gray-200"></div>
                                                    <span class="text-xs font-sans text-gray-400 font-bold">
                                                        {format!("{} ICONS", group.1.len())}
                                                    </span>
                                                </div>

                                                <div class="icons-grid">
                                                    {
                                                        let prefix_for_inner = prefix_for_grid.clone();
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
                                                                            class="icon-item group"
                                                                            on:click=move |_| set_selected_icon.set(Some(name_clone.clone()))
                                                                        >
                                                                            <div class="icon-preview-box group-hover:bg-gray-800 group-hover:border-gray-600 group-hover:text-white transition-colors">
                                                                                <img src=img_url alt=icon_name.clone() loading="lazy" width="32" height="32" />
                                                                            </div>
                                                                            <div class="w-full text-left">
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
                                        </For>
                                    </div>
                                    
                                    // ── Drawer ───────────────────────────────────────
                                    {move || {
                                        let is_open = selected_icon.get().is_some();
                                        let current_icon = selected_icon.get().unwrap_or_default();
                                        let p = prefix_for_drawer.clone();
                                        
                                        view! {
                                            <div class=format!("drawer {}", if is_open { "open" } else { "" })>
                                                <Show when=move || is_open>
                                                    <IconDetail 
                                                        prefix=p.clone()
                                                        name=current_icon.clone()
                                                        on_close=Callback::new(move |_| set_selected_icon.set(None)) 
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
                    })}
                </Suspense>
            </main>
        </div>
    }
}
