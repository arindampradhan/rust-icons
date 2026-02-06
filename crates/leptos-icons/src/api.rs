use std::collections::HashMap;

use gloo_net::http::Request;
use rust_icons_core::types::{
    CollectionInfo, CollectionInfoRaw, CollectionResponse, IconifyResponse, ResolvedIcon,
};

const BASE_URL: &str = "https://api.iconify.design";

/// Fetch all collections from the Iconify API.
pub async fn fetch_collections() -> Result<Vec<CollectionInfo>, String> {
    let response = Request::get(&format!("{BASE_URL}/collections"))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let resp: HashMap<String, CollectionInfoRaw> = response
        .json()
        .await
        .map_err(|e| format!("JSON error: {e}"))?;

    let mut collections: Vec<CollectionInfo> = resp
        .into_iter()
        .filter(|(_, raw)| !raw.hidden.unwrap_or(false))
        .map(|(id, raw)| CollectionInfo::from_raw(id, raw))
        .collect();

    collections.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(collections)
}

/// Fetch all icons in a collection (list of icon names).
pub async fn fetch_collection_icons(prefix: &str) -> Result<CollectionResponse, String> {
    let response = Request::get(&format!("{BASE_URL}/collection?prefix={prefix}"))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!(
            "Collection '{}' not found (HTTP {})",
            prefix,
            response.status()
        ));
    }

    response
        .json()
        .await
        .map_err(|e| format!("JSON error: {e}"))
}

/// Fetch raw SVG for a single icon.
#[allow(dead_code)]
pub async fn fetch_svg(prefix: &str, name: &str) -> Result<String, String> {
    let response = Request::get(&format!("{BASE_URL}/{prefix}/{name}.svg"))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!(
            "Icon '{prefix}:{name}' not found (HTTP {})",
            response.status()
        ));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Text error: {e}"))
}

/// Fetch icon data (body, dimensions) for snippet generation.
pub async fn fetch_icon_data(prefix: &str, name: &str) -> Result<ResolvedIcon, String> {
    let response = Request::get(&format!("{BASE_URL}/{prefix}.json?icons={name}"))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!(
            "Icon '{prefix}:{name}' not found (HTTP {})",
            response.status()
        ));
    }

    let data: IconifyResponse = response
        .json()
        .await
        .map_err(|e| format!("JSON error: {e}"))?;

    ResolvedIcon::from_response(&data, name)
        .ok_or_else(|| format!("Icon '{name}' not found in response"))
}
