use leptos::prelude::*;
use rust_icons_core::svg::iconify_img_url;
use rust_icons_core::types::CollectionInfo;

#[component]
pub fn CollectionCard(collection: CollectionInfo) -> impl IntoView {
    let href = format!("/collection/{}", collection.id);

    // Fallback if license is missing
    let license_spdx = collection
        .license
        .as_ref()
        .and_then(|l| l.spdx.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    let author_name = collection
        .author
        .as_ref()
        .map(|a| a.name.clone())
        .unwrap_or_default();

    let CollectionInfo {
        id, name, total, ..
    } = collection;

    let sample_urls: Vec<String> = collection
        .samples
        .iter()
        .take(3)
        .map(|s| iconify_img_url(&id, s))
        .collect();

    view! {
        <a href=href class="icon-card group">
            <div class="card-header">
                <h3 class="card-title">{name}</h3>
                <div class="card-badge">{license_spdx}</div>
            </div>

            <div class="card-preview">
                <div class="preview-pattern"></div>
                <div class="preview-samples">
                    {sample_urls.into_iter().map(|url| {
                        view! { <img src=url alt="" loading="lazy" /> }
                    }).collect_view()}
                </div>
            </div>

            <div class="card-footer">
                <span>{total} " icons"</span>
                <span class="tracking-widest">{author_name}</span>
            </div>
        </a>
    }
}
