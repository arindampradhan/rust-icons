use crate::types::CollectionInfo;

/// Case-insensitive substring filter for collections.
///
/// Matches against collection name, ID, and category.
#[must_use]
pub fn filter_collections<'a>(
    collections: &'a [CollectionInfo],
    query: &str,
) -> Vec<&'a CollectionInfo> {
    if query.is_empty() {
        return collections.iter().collect();
    }
    let q = query.to_lowercase();
    collections
        .iter()
        .filter(|c| {
            c.name.to_lowercase().contains(&q)
                || c.id.to_lowercase().contains(&q)
                || c.category.to_lowercase().contains(&q)
        })
        .collect()
}

/// Case-insensitive substring filter for icon names.
#[must_use]
pub fn filter_icons<'a>(icons: &'a [String], query: &str) -> Vec<&'a String> {
    if query.is_empty() {
        return icons.iter().collect();
    }
    let q = query.to_lowercase();
    icons
        .iter()
        .filter(|name| name.to_lowercase().contains(&q))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CollectionInfo;

    fn make_collection(id: &str, name: &str, category: &str) -> CollectionInfo {
        CollectionInfo {
            id: id.into(),
            name: name.into(),
            total: 10,
            author: None,
            license: None,
            samples: vec![],
            category: category.into(),
            palette: false,
            hidden: false,
        }
    }

    #[test]
    fn empty_query_returns_all() {
        let cols = vec![make_collection("a", "A", "General")];
        assert_eq!(filter_collections(&cols, "").len(), 1);
    }

    #[test]
    fn filters_by_name() {
        let cols = vec![
            make_collection("mdi", "Material Design Icons", "General"),
            make_collection("fa", "Font Awesome", "General"),
        ];
        let result = filter_collections(&cols, "material");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "mdi");
    }

    #[test]
    fn filters_icons_case_insensitive() {
        let icons: Vec<String> = vec!["arrow-up".into(), "arrow-down".into(), "check".into()];
        let result = filter_icons(&icons, "ARROW");
        assert_eq!(result.len(), 2);
    }
}
