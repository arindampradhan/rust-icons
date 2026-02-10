use leptos::prelude::*;
use rust_icons_core::search::search_collections;
use rust_icons_core::types::CollectionInfo;

use crate::api;
use crate::components::collection_card::CollectionCard;

#[component]
pub fn HomePage() -> impl IntoView {
    let collections = LocalResource::new(api::fetch_collections);
    let (search, set_search) = signal(String::new());

    view! {
        <div class="page-container">
            <div class="paper-sheet">
                // ── Masthead ─────────────────────────────────────
                <header class="masthead">
                    <div class="masthead-meta">
                        <span>"Vol. CCLVI No. 104"</span>
                        <span class="flex items-center gap-2">Rust Icons | Daily Edition</span>
                        <span>"$4.00"</span>
                    </div>

                    // <h1 class="masthead-title">"Rust Icons"</h1>
                    // <div class="font-serif italic text-lg mb-6">"All the icons that are fit to print."</div>

                    <div class="search-wrapper max-w-2xl mx-auto">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search categories..."
                            prop:value=search
                            on:input:target=move |ev| set_search.set(ev.target().value())
                        />
                        <div class="search-icon">
                            // Simple SVG search icon
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="11" cy="11" r="8"></circle>
                                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                            </svg>
                        </div>
                    </div>

                    <nav class="masthead-nav">
                        <span class="nav-item">"All"</span>
                        <span class="nav-item">"Recent"</span>
                        <span class="nav-item">"Material"</span>
                        <span class="nav-item">"UI 24px"</span>
                        <span class="nav-item">"Logos"</span>
                        <span class="nav-item">"Emoji"</span>
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
