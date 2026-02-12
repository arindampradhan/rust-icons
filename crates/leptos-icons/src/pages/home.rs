use leptos::prelude::*;
use rust_icons_core::search::search_collections;
use rust_icons_core::types::CollectionInfo;

use crate::api;
use crate::components::collection_card::CollectionCard;
use crate::components::theme_toggle::ThemeToggle;

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
                        <span class="masthead-actions">
                            // GitHub
                            <a href="https://github.com/arindampradhan/rust-icons" target="_blank" class="action-btn" title="GitHub">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>
                            </a>
                            // Settings
                            // <button class="action-btn" title="Settings">
                            //    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
                            // </button>
                            // Light/Dark mode
                            <ThemeToggle />
                        </span>
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

                                        // Custom sort: prioritize "Recent" first, push "Archive / Unmaintained" to end
                                        categories.sort_by(|a, b| {
                                            let a_lower = a.0.to_lowercase();
                                            let b_lower = b.0.to_lowercase();

                                            let a_is_recent = a_lower == "recent";
                                            let b_is_recent = b_lower == "recent";
                                            let a_is_archive =
                                                a_lower.contains("archive") || a_lower.contains("unmaintained");
                                            let b_is_archive =
                                                b_lower.contains("archive") || b_lower.contains("unmaintained");

                                            match (a_is_recent, b_is_recent, a_is_archive, b_is_archive) {
                                                // Recent always comes first
                                                (true, false, _, _) => std::cmp::Ordering::Less,
                                                (false, true, _, _) => std::cmp::Ordering::Greater,
                                                // Archive/unmaintained goes to end (unless both are archive)
                                                (false, false, true, false) => std::cmp::Ordering::Greater,
                                                (false, false, false, true) => std::cmp::Ordering::Less,
                                                // Otherwise sort alphabetically
                                                _ => a.0.cmp(&b.0),
                                            }
                                        });

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
