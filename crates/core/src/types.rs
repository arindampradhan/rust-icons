use std::collections::HashMap;

use serde::Deserialize;

/// Raw collection info as returned by the Iconify `/collections` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CollectionInfoRaw {
    pub name: String,
    pub total: Option<u32>,
    pub author: Option<Author>,
    pub license: Option<License>,
    pub samples: Option<Vec<String>>,
    pub category: Option<String>,
    pub palette: Option<bool>,
    pub hidden: Option<bool>,
    pub height: Option<HeightValue>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum HeightValue {
    Single(u32),
    Multiple(Vec<u32>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct License {
    pub title: String,
    pub url: Option<String>,
    pub spdx: Option<String>,
}

/// Processed collection info for the UI.
#[derive(Debug, Clone)]
pub struct CollectionInfo {
    pub id: String,
    pub name: String,
    pub total: u32,
    pub author: Option<Author>,
    pub license: Option<License>,
    pub samples: Vec<String>,
    pub category: String,
    pub palette: bool,
    pub hidden: bool,
}

impl CollectionInfo {
    /// Build from the raw API response entry.
    #[must_use]
    pub fn from_raw(id: String, raw: CollectionInfoRaw) -> Self {
        Self {
            id,
            name: raw.name,
            total: raw.total.unwrap_or(0),
            author: raw.author,
            license: raw.license,
            samples: raw.samples.unwrap_or_default(),
            category: raw.category.unwrap_or_else(|| "Uncategorized".to_string()),
            palette: raw.palette.unwrap_or(false),
            hidden: raw.hidden.unwrap_or(false),
        }
    }
}

/// Response from `GET /{prefix}.json?icons=...` — specific icons data.
#[derive(Debug, Clone, Deserialize)]
pub struct IconifyResponse {
    pub prefix: String,
    pub icons: HashMap<String, IconData>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(default)]
    pub categories: Option<HashMap<String, Vec<String>>>,
}

/// Response from `GET /collection?prefix={prefix}` — list of icons in a collection.
#[derive(Debug, Clone, Deserialize)]
pub struct CollectionResponse {
    pub prefix: String,
    pub total: u32,
    pub title: Option<String>,
    #[serde(default)]
    pub uncategorized: Vec<String>,
    #[serde(default)]
    pub categories: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub hidden: Vec<String>,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

impl CollectionResponse {
    /// Get all visible icon names (uncategorized + categorized, deduplicated).
    #[must_use]
    pub fn all_icon_names(&self) -> Vec<String> {
        let mut names: std::collections::HashSet<String> =
            self.uncategorized.iter().cloned().collect();
        for icons in self.categories.values() {
            names.extend(icons.iter().cloned());
        }
        let mut result: Vec<String> = names.into_iter().collect();
        result.sort();
        result
    }
}

/// Individual icon data within an icon set.
#[derive(Debug, Clone, Deserialize)]
pub struct IconData {
    pub body: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Resolved icon with all fields filled in, ready to render.
#[derive(Debug, Clone)]
pub struct ResolvedIcon {
    pub prefix: String,
    pub name: String,
    pub body: String,
    pub width: u32,
    pub height: u32,
}

impl ResolvedIcon {
    /// Resolve an icon from the collection response, falling back to set-level defaults.
    #[must_use]
    pub fn from_response(resp: &IconifyResponse, name: &str) -> Option<Self> {
        let data = resp.icons.get(name)?;
        Some(Self {
            prefix: resp.prefix.clone(),
            name: name.to_string(),
            body: data.body.clone(),
            width: data.width.or(resp.width).unwrap_or(24),
            height: data.height.or(resp.height).unwrap_or(24),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_info_defaults() {
        let raw = CollectionInfoRaw {
            name: "Test".into(),
            total: None,
            author: None,
            license: None,
            samples: None,
            category: None,
            palette: None,
            hidden: None,
            height: None,
        };
        let info = CollectionInfo::from_raw("test".into(), raw);
        assert_eq!(info.total, 0);
        assert_eq!(info.category, "Uncategorized");
        assert!(!info.palette);
        assert!(!info.hidden);
    }

    #[test]
    fn resolved_icon_falls_back_to_set_defaults() {
        let mut icons = HashMap::new();
        icons.insert(
            "arrow".to_string(),
            IconData {
                body: "<path/>".into(),
                width: None,
                height: None,
            },
        );
        let resp = IconifyResponse {
            prefix: "mdi".into(),
            icons,
            width: Some(24),
            height: Some(24),
            categories: None,
        };
        let resolved = ResolvedIcon::from_response(&resp, "arrow").unwrap();
        assert_eq!(resolved.width, 24);
        assert_eq!(resolved.height, 24);
    }
}
