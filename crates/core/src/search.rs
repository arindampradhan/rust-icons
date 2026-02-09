use crate::types::CollectionInfo;

// ---------------------------------------------------------------------------
// Aliases — ported from icones/src/data/search-alias.ts
// ---------------------------------------------------------------------------

const ALIASES: &[&[&str]] = &[
    &["account", "person", "profile", "user"],
    &["add", "create", "new", "plus"],
    &["alert", "bell", "notification", "notify", "reminder"],
    &["approve", "like", "recommend", "thumbs-up"],
    &["left", "previous"],
    &["next", "right"],
    &["attach", "connect", "link"],
    &["bag", "basket", "cart"],
    &["bookmark", "tag", "label"],
    &["building", "home", "house"],
    &["calendar", "date", "event"],
    &["cancel", "close"],
    &["delete", "remove", "trash"],
    &["chat", "conversation", "message"],
    &["clock", "time", "timer", "alarm"],
    &["cog", "gear", "preferences", "settings"],
    &["directory", "folder"],
    &["disapprove", "dislike", "thumbs-down"],
    &["document", "file", "paper"],
    &["earth", "globe", "world", "planet", "global"],
    &["email", "envelope", "mail"],
    &["eye", "view", "visible"],
    &["favorite", "heart", "love"],
    &["feed", "rss", "subscribe", "subscription"],
    &["list", "menu"],
    &["lock", "secure", "security"],
    &["unlock", "lock-open"],
    &["log-in", "login", "sign-in"],
    &["log-out", "logout", "sign-out"],
    &["magnifier", "search", "find", "magnify"],
    &["photo", "picture", "image"],
    &["refresh", "reload", "update", "sync"],
    &["speaker", "audio", "volume", "sound"],
    &["speed", "fast"],
    &["accessibility", "ally", "a11y"],
    &["edit", "pen", "pencil", "write"],
    &["moon", "night", "dark"],
    &["sun", "day"],
    &["bulb", "idea"],
    &["pin", "location", "map", "marker"],
    &["bot", "robot", "android"],
    &["db", "database"],
    &["external", "launch"],
    &["airplane", "flight"],
    &["chart", "graph"],
    &["monitor", "screen"],
    &["video", "film"],
    &["support", "help", "question"],
    &["mute", "silence", "sound-off", "volume-off"],
    &["code", "development", "program", "terminal", "braces"],
    &["phone", "call"],
    &["car", "vehicle", "transport", "taxi"],
];

/// Return all alias synonyms for `word` (excluding `word` itself).
fn expand_aliases(word: &str) -> Vec<&'static str> {
    for group in ALIASES {
        if group.contains(&word) {
            return group.iter().copied().filter(|&w| w != word).collect();
        }
    }
    Vec::new()
}

/// Expand a multi-word query into all candidate query strings.
///
/// Each word is expanded with its aliases, then candidates are produced by
/// taking each alias individually as a replacement for the original word.
/// The original query is always the first candidate.
fn expand_query(query: &str) -> Vec<String> {
    let words: Vec<&str> = query.split_whitespace().collect();
    if words.is_empty() {
        return vec![];
    }

    let mut candidates = vec![words.join(" ")];

    for (i, &word) in words.iter().enumerate() {
        for alias in expand_aliases(word) {
            let mut replaced = words.clone();
            replaced[i] = alias;
            candidates.push(replaced.join(" "));
        }
    }

    candidates
}

// ---------------------------------------------------------------------------
// Fuzzy scorer
// ---------------------------------------------------------------------------

const CONSECUTIVE_BONUS: i32 = 4;
const WORD_BOUNDARY_BONUS: i32 = 8;
const PREFIX_BONUS: i32 = 12;
const GAP_PENALTY: i32 = -1;
const EXACT_MATCH_BONUS: i32 = 20;

/// Score how well `pattern` fuzzy-matches `text`.
///
/// Returns `None` if not all pattern characters appear sequentially in `text`.
/// Higher scores indicate better matches.
fn fuzzy_score(pattern: &str, text: &str) -> Option<i32> {
    if pattern.is_empty() {
        return Some(0);
    }

    let pattern_lower: Vec<char> = pattern.chars().flat_map(char::to_lowercase).collect();
    let text_lower: Vec<char> = text.chars().flat_map(char::to_lowercase).collect();

    let mut score: i32 = 0;
    let mut pi = 0; // pattern index
    let mut prev_match_idx: Option<usize> = None;

    for (ti, &tc) in text_lower.iter().enumerate() {
        if pi < pattern_lower.len() && tc == pattern_lower[pi] {
            // Consecutive match bonus
            if let Some(prev) = prev_match_idx {
                if ti == prev + 1 {
                    score += CONSECUTIVE_BONUS;
                } else {
                    // Gap penalty for skipped chars — icon names are short, cast is safe
                    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                    let gap = (ti - prev - 1) as i32;
                    score += GAP_PENALTY * gap;
                }
            }

            // Word-boundary bonus (start of text or after `-` / `_`)
            if ti == 0 {
                score += PREFIX_BONUS;
            } else {
                let prev_char = text_lower[ti - 1];
                if prev_char == '-' || prev_char == '_' {
                    score += WORD_BOUNDARY_BONUS;
                }
            }

            prev_match_idx = Some(ti);
            pi += 1;
        }
    }

    if pi == pattern_lower.len() {
        // Exact match bonus when pattern covers the full text
        if pattern_lower.len() == text_lower.len() {
            score += EXACT_MATCH_BONUS;
        }
        Some(score)
    } else {
        None
    }
}

/// Score a multi-word query against `text`.
///
/// Splits query on whitespace, requires all words to match, sums scores.
fn fuzzy_score_multi(query: &str, text: &str) -> Option<i32> {
    let words: Vec<&str> = query.split_whitespace().collect();
    if words.is_empty() {
        return Some(0);
    }

    let mut total = 0i32;
    for word in &words {
        match fuzzy_score(word, text) {
            Some(s) => total += s,
            None => return None,
        }
    }
    Some(total)
}

// ---------------------------------------------------------------------------
// Public search API
// ---------------------------------------------------------------------------

/// Fuzzy-search icon names, returning results sorted by relevance.
///
/// Empty query returns all icons in original order.
#[must_use]
pub fn search_icons(icons: &[String], query: &str) -> Vec<String> {
    if query.is_empty() {
        return icons.to_vec();
    }

    let q = query.trim().to_lowercase();
    let candidates = expand_query(&q);

    let mut scored: Vec<(i32, &String)> = icons
        .iter()
        .filter_map(|name| {
            let best = candidates
                .iter()
                .filter_map(|c| fuzzy_score_multi(c, name))
                .max()?;
            Some((best, name))
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, name)| name.clone()).collect()
}

/// Fuzzy-search collections, returning results sorted by relevance.
///
/// Scores against name, ID, and category — takes the best field score.
/// Empty query returns all collections in original order.
#[must_use]
pub fn search_collections<'a>(
    collections: &'a [CollectionInfo],
    query: &str,
) -> Vec<&'a CollectionInfo> {
    if query.is_empty() {
        return collections.iter().collect();
    }

    let q = query.trim().to_lowercase();
    let candidates = expand_query(&q);

    let mut scored: Vec<(i32, &CollectionInfo)> = collections
        .iter()
        .filter_map(|c| {
            let best = candidates
                .iter()
                .filter_map(|cand| {
                    let scores = [
                        fuzzy_score_multi(cand, &c.name),
                        fuzzy_score_multi(cand, &c.id),
                        fuzzy_score_multi(cand, &c.category),
                    ];
                    scores.into_iter().flatten().max()
                })
                .max()?;
            Some((best, c))
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, c)| c).collect()
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

    // -- fuzzy_score tests --

    #[test]
    fn exact_match_scores_high() {
        let s = fuzzy_score("arrow", "arrow").unwrap();
        assert!(s > 0);
    }

    #[test]
    fn fuzzy_prefix_matches() {
        assert!(fuzzy_score("arw", "arrow").is_some());
    }

    #[test]
    fn no_match_returns_none() {
        assert!(fuzzy_score("xyz", "arrow").is_none());
    }

    #[test]
    fn case_insensitive() {
        assert!(fuzzy_score("ARW", "arrow").is_some());
    }

    #[test]
    fn word_boundary_bonus() {
        let boundary = fuzzy_score("au", "arrow-up").unwrap();
        let no_boundary = fuzzy_score("au", "xaxux").unwrap();
        assert!(boundary > no_boundary, "word boundary should score higher");
    }

    #[test]
    fn consecutive_beats_scattered() {
        let consecutive = fuzzy_score("arr", "arrow").unwrap();
        let scattered = fuzzy_score("arr", "axrxrxow").unwrap();
        assert!(
            consecutive > scattered,
            "consecutive matches should score higher"
        );
    }

    // -- alias tests --

    #[test]
    fn alias_expansion_finds_synonyms() {
        let aliases = expand_aliases("trash");
        assert!(aliases.contains(&"delete"));
        assert!(aliases.contains(&"remove"));
        assert!(!aliases.contains(&"trash"));
    }

    #[test]
    fn alias_expansion_unknown_word() {
        assert!(expand_aliases("xyzzynonexistent").is_empty());
    }

    #[test]
    fn query_expansion() {
        let candidates = expand_query("trash");
        assert!(candidates.contains(&"trash".to_string()));
        assert!(candidates.contains(&"delete".to_string()));
        assert!(candidates.contains(&"remove".to_string()));
    }

    // -- search_icons tests --

    #[test]
    fn empty_query_returns_all_icons() {
        let icons: Vec<String> = vec!["arrow-up".into(), "check".into()];
        assert_eq!(search_icons(&icons, "").len(), 2);
    }

    #[test]
    fn fuzzy_search_finds_arrow() {
        let icons: Vec<String> = vec![
            "arrow-up".into(),
            "arrow-down".into(),
            "check".into(),
            "close".into(),
        ];
        let results = search_icons(&icons, "arw");
        assert!(results.iter().any(|n| n.contains("arrow")));
        assert!(!results.iter().any(|n| n == "check"));
    }

    #[test]
    fn alias_search_finds_delete_via_trash() {
        let icons: Vec<String> = vec![
            "delete".into(),
            "remove-circle".into(),
            "trash-bin".into(),
            "star".into(),
        ];
        let results = search_icons(&icons, "trash");
        assert!(results.iter().any(|n| n == "delete"));
        assert!(results.iter().any(|n| n == "remove-circle"));
        assert!(!results.iter().any(|n| n == "star"));
    }

    #[test]
    fn results_sorted_by_relevance() {
        let icons: Vec<String> = vec!["x-arrow-yz".into(), "arrow".into(), "arrow-up".into()];
        let results = search_icons(&icons, "arrow");
        // Exact match "arrow" should be first
        assert_eq!(results[0], "arrow");
    }

    // -- search_collections tests --

    #[test]
    fn empty_query_returns_all_collections() {
        let cols = vec![make_collection("a", "A", "General")];
        assert_eq!(search_collections(&cols, "").len(), 1);
    }

    #[test]
    fn searches_by_name() {
        let cols = vec![
            make_collection("mdi", "Material Design Icons", "General"),
            make_collection("fa", "Font Awesome", "General"),
        ];
        let result = search_collections(&cols, "material");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "mdi");
    }

    #[test]
    fn fuzzy_collection_search() {
        let cols = vec![
            make_collection("mdi", "Material Design Icons", "General"),
            make_collection("fa", "Font Awesome", "General"),
        ];
        let result = search_collections(&cols, "mtrl");
        assert!(
            result.iter().any(|c| c.id == "mdi"),
            "fuzzy should match 'mdi' for 'mtrl'"
        );
    }
}
