use crate::types::ResolvedIcon;

/// Build a complete SVG string from a resolved icon.
pub fn build_svg(icon: &ResolvedIcon) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">{}</svg>"#,
        icon.width, icon.height, icon.width, icon.height, icon.body,
    )
}

/// URL for the Iconify CDN SVG endpoint.
pub fn iconify_svg_url(prefix: &str, name: &str) -> String {
    format!("https://api.iconify.design/{prefix}/{name}.svg")
}

/// URL for the Iconify CDN to use in `<img>` tags.
///
/// Provides a smaller, pre-rendered SVG that's ideal for grid thumbnails.
pub fn iconify_img_url(prefix: &str, name: &str) -> String {
    format!("https://api.iconify.design/{prefix}/{name}.svg?height=1.2em")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_svg_output() {
        let icon = ResolvedIcon {
            prefix: "mdi".into(),
            name: "arrow".into(),
            body: "<path d=\"M5 12h14\"/>".into(),
            width: 24,
            height: 24,
        };
        let svg = build_svg(&icon);
        assert!(svg.contains("viewBox=\"0 0 24 24\""));
        assert!(svg.contains("<path"));
    }

    #[test]
    fn img_url_format() {
        let url = iconify_img_url("mdi", "home");
        assert_eq!(url, "https://api.iconify.design/mdi/home.svg?height=1.2em");
    }
}
