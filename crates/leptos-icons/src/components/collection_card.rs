use leptos::prelude::*;
use rust_icons_core::svg::iconify_img_url;
use rust_icons_core::types::CollectionInfo;

#[component]
pub fn CollectionCard(collection: CollectionInfo) -> impl IntoView {
    let href = format!("/collection/{}", collection.id);
    let author_name = collection
        .author
        .as_ref()
        .map(|a| a.name.clone())
        .unwrap_or_default();
    let license_title = collection
        .license
        .as_ref()
        .map(|l| l.title.clone())
        .unwrap_or_default();
    let total = collection.total;
    let name = collection.name.clone();
    let prefix = collection.id.clone();
    let samples = collection.samples.clone();

    view! {
        <a class="collection-card" href=href>
            <div class="card-info">
                <div class="card-name">{name}</div>
                <div class="card-meta">
                    <span>{author_name}</span>
                    <span>{license_title}</span>
                    <span>{total} " icons"</span>
                </div>
            </div>
            <div class="card-samples">
                {samples.into_iter().take(3).map(|icon_name| {
                    let url = iconify_img_url(&prefix, &icon_name);
                    view! { <img src=url alt="" loading="lazy" /> }
                }).collect::<Vec<_>>()}
            </div>
        </a>
    }
}
