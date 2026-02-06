use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rust_icons_core::search::filter_icons;

use crate::api;
use crate::components::icon_detail::IconDetail;
use crate::components::icon_grid::IconGrid;
use crate::components::BottomDrawer;
use crate::components::navbar::Navbar;
use crate::components::search_bar::SearchBar;

#[component]
pub fn CollectionPage() -> impl IntoView {
    let params = use_params_map();
    // Use read_untracked for initial title since route params don't change within a page
    let initial_id = params.read_untracked().get("id").unwrap_or_default();
    let id = move || params.read().get("id").unwrap_or_default();

    let icons_resource = LocalResource::new(move || {
        let prefix = id();
        async move { api::fetch_collection_icons(&prefix).await }
    });

    let (search, set_search) = signal(String::new());
    let (selected_icon, set_selected_icon) = signal(None::<String>);

    view! {
        <Navbar back_href="/" title=initial_id.clone() />
        <div class="collection-page">
            <Suspense fallback=|| view! { <div class="loading">"Loading icons..."</div> }>
                {move || Suspend::new(async move {
                    let prefix = id();
                    match icons_resource.await {
                        Ok(resp) => {
                            let icon_names = resp.all_icon_names();
                            let total = icon_names.len();
                            let all_icons = icon_names.clone();

                            let filtered_icons = Signal::derive(move || {
                                let q = search.get();
                                if q.is_empty() {
                                    all_icons.clone()
                                } else {
                                    filter_icons(&all_icons, &q)
                                        .into_iter()
                                        .cloned()
                                        .collect()
                                }
                            });

                            let prefix_for_grid = prefix.clone();
                            let prefix_for_detail = prefix.clone();

                            view! {
                                <div class="collection-header">
                                    <h2>{prefix.clone()}</h2>
                                    <span class="icon-count">{total} " icons"</span>
                                    <div class="collection-search">
                                        <SearchBar
                                            value=search
                                            set_value=set_search
                                            placeholder="Filter icons..."
                                        />
                                    </div>
                                </div>
                                <IconGrid
                                    prefix=prefix_for_grid
                                    icons=filtered_icons
                                    on_select=Callback::new(move |name: String| {
                                        set_selected_icon.set(Some(name));
                                    })
                                />
                                <Show when=move || selected_icon.get().is_some()>
                                    {
                                        let icon_name = selected_icon.get().unwrap_or_default();
                                        let prefix = prefix_for_detail.clone();
                                        view! {
                                            <BottomDrawer on_close=Callback::new(move |()| set_selected_icon.set(None))>
                                                <IconDetail
                                                    prefix=prefix
                                                    name=icon_name
                                                    on_close=Callback::new(move |()| set_selected_icon.set(None))
                                                />
                                            </BottomDrawer>
                                        }
                                    }
                                </Show>
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
