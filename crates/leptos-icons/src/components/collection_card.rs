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

    let CollectionInfo {
        id, name, total, ..
    } = collection;

    // Use the first sample icon for the preview, or a default if none
    let preview_url = iconify_img_url(
        &id,
        &collection.samples.first().cloned().unwrap_or_default(),
    );

    view! {
        <a href=href class="icon-card group">
            <div class="card-header">
                <h3 class="card-title">{name}</h3>
                <div class="card-badge">"SVG"</div>
            </div>

            <div class="card-preview">
                <div class="preview-pattern"></div>
                <img src=preview_url alt=id.clone() loading="lazy" />
            </div>

            <div class="card-footer">
                <span>{total} " icons"</span>
                <span class="uppercase tracking-widest">{license_spdx}</span>
            </div>
        </a>
    }
}
