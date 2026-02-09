use leptos::prelude::*;
use rust_icons_core::search::search_collections;
use rust_icons_core::types::CollectionInfo;

use crate::api;
use crate::components::collection_grid::CollectionGrid;
use crate::components::navbar::Navbar;
use crate::components::search_bar::SearchBar;

#[component]
pub fn HomePage() -> impl IntoView {
    let collections = LocalResource::new(api::fetch_collections);
    let (search, set_search) = signal(String::new());

    view! {
        <Navbar />
        <div class="home-page">
            <div class="home-search">
                <SearchBar
                    value=search
                    set_value=set_search
                    placeholder="Search collections..."
                />
            </div>
            <Suspense fallback=|| view! { <div class="loading">"Loading collections..."</div> }>
                {move || Suspend::new(async move {
                    match collections.await {
                        Ok(all_collections) => {
                            // Use Signal::derive to properly track the search signal reactively
                            let grouped = Signal::derive(move || {
                                let filtered: Vec<&CollectionInfo> =
                                    search_collections(&all_collections, &search.get());

                                // Group by category
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
                                    <div class="category-section">
                                        <div class="category-title">{entry.0.clone()}</div>
                                        <CollectionGrid collections=entry.1.clone() />
                                    </div>
                                </For>
                            }.into_any()
                        }
                        Err(e) => view! {
                            <div class="empty-state">{format!("Failed to load: {e}")}</div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}
