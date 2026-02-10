use leptos::prelude::*;
use rust_icons_core::search::search_collections;
use rust_icons_core::types::CollectionInfo;

use crate::api;
use crate::components::collection_card::CollectionCard;

#[component]
pub fn HomePage() -> impl IntoView {
    let collections = LocalResource::new(api::fetch_collections);
    let (search, set_search) = signal(String::new());
    let (active_filter, set_active_filter) = signal(None::<String>);

    view! {
        <div class="page-container">
            <div class="paper-sheet">
                // ── Masthead ─────────────────────────────────────
                <header class="masthead">
                    <div class="masthead-meta">
                        <span>"Vol. CCLVI No. 104"</span>
                        <span class="masthead-meta-center">"🦀  Rust Icons | Daily Edition"</span>
                        <span>"$4.00"</span>
                    </div>

                    <div class="search-wrapper">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search categories..."
                            prop:value=search
                            on:input:target=move |ev| set_search.set(ev.target().value())
                        />
                        <div class="search-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="11" cy="11" r="8"></circle>
                                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                            </svg>
                        </div>
                    </div>

                    <nav class="masthead-nav">
                        {
                            let filters: Vec<(&str, Option<&str>)> = vec![
                                ("All", None),
                                ("Material", Some("Material")),
                                ("UI 24px", Some("UI 24px")),
                                ("Logos", Some("Logos")),
                                ("Emoji", Some("Emoji")),
                                ("Thematic", Some("Thematic")),
                            ];
                            filters.into_iter().map(|(label, cat)| {
                                let cat_value = cat.map(String::from);
                                let cat_for_class = cat_value.clone();
                                view! {
                                    <button
                                        class=move || {
                                            if active_filter.get() == cat_for_class {
                                                "nav-item active"
                                            } else {
                                                "nav-item"
                                            }
                                        }
                                        on:click=move |_| set_active_filter.set(cat_value.clone())
                                    >{label}</button>
                                }
                            }).collect_view()
                        }
                    </nav>
                </header>

                // ── Content ──────────────────────────────────────
                <div class="content-wrapper">
                    <Suspense fallback=|| view! { <div class="loading">"Loading the press..."</div> }>
                        {move || Suspend::new(async move {
                            match collections.await {
                                Ok(all_collections) => {
                                    let grouped = Signal::derive(move || {
                                        let filtered: Vec<&CollectionInfo> =
                                            search_collections(&all_collections, &search.get());

                                        // Apply category filter
                                        let filtered: Vec<&CollectionInfo> = match active_filter.get() {
                                            None => filtered,
                                            Some(cat) => filtered.into_iter()
                                                .filter(|c| c.category == cat)
                                                .collect(),
                                        };

                                        let mut categories: Vec<(String, Vec<CollectionInfo>)> = Vec::new();
                                        for c in &filtered {
                                            if let Some(group) = categories.iter_mut().find(|(cat, _)| cat == &c.category) {
                                                group.1.push((*c).clone());
                                            } else {
                                                categories.push((c.category.clone(), vec![(*c).clone()]));
                                            }
                                        }
                                        categories.sort_by(|a, b| a.0.cmp(&b.0));
                                        categories
                                    });

                                    view! {
                                        <For
                                            each=move || grouped.get()
                                            key=|(cat, _)| cat.clone()
                                            let:entry
                                        >
                                            <section class="mb-12">
                                                <div class="section-header">
                                                    <h2 class="section-title">{entry.0.clone()}</h2>
                                                    <span class="section-meta">"Section " {entry.0.chars().next().unwrap_or('A')}</span>
                                                </div>
                                                <div class="cards-grid">
                                                    <For
                                                        each=move || entry.1.clone()
                                                        key=|c| c.id.clone()
                                                        let:collection
                                                    >
                                                        <CollectionCard collection=collection />
                                                    </For>
                                                </div>
                                            </section>
                                        </For>
                                    }.into_any()
                                }
                                Err(e) => view! {
                                    <div class="empty-state">{format!("Failed to load edition: {e}")}</div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
