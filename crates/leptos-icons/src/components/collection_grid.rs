use leptos::prelude::*;
use rust_icons_core::types::CollectionInfo;

use super::collection_card::CollectionCard;

#[component]
pub fn CollectionGrid(collections: Vec<CollectionInfo>) -> impl IntoView {
    view! {
        <div class="collection-grid">
            {collections.into_iter().map(|c| {
                view! { <CollectionCard collection=c /> }
            }).collect::<Vec<_>>()}
        </div>
    }
}
