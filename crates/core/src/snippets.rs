//! Snippet generation for various output formats.
//!
//! Converts SVG icons to framework-specific code snippets for:
//! - Rust frameworks: Leptos, Yew, Dioxus
//! - Web frameworks: Vue, React, Svelte, Solid, Qwik, Astro
//! - Data formats: SVG, Base64, Data URL, CSS

use crate::types::ResolvedIcon;

/// Snippet output format category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnippetCategory {
    /// Raw SVG and data formats
    Snippets,
    /// Rust framework components
    Rust,
    /// Web framework components
    Components,
    /// URLs and links
    Links,
}

/// All supported snippet types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnippetType {
    // Snippets
    Svg,
    SvgSymbol,
    Iconify,
    Jsx,

    // Rust frameworks
    Leptos,
    Yew,
    Dioxus,

    // Web components
    Vue,
    VueTs,
    React,
    ReactTs,
    Svelte,
    Qwik,
    Solid,
    Astro,

    // Links
    Url,
    DataUrl,
    Base64,
    CssBackground,
}

impl SnippetType {
    /// Display name for UI.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Svg => "SVG",
            Self::SvgSymbol => "SVG Symbol",
            Self::Iconify => "Iconify",
            Self::Jsx => "JSX",
            Self::Leptos => "Leptos",
            Self::Yew => "Yew",
            Self::Dioxus => "Dioxus",
            Self::Vue | Self::VueTs => "Vue",
            Self::React | Self::ReactTs => "React",
            Self::Svelte => "Svelte",
            Self::Qwik => "Qwik",
            Self::Solid => "Solid",
            Self::Astro => "Astro",
            Self::Url => "URL",
            Self::DataUrl => "Data URL",
            Self::Base64 => "Base64",
            Self::CssBackground => "CSS",
        }
    }

    /// Optional tag suffix (e.g., "TS" for TypeScript variants).
    #[must_use]
    pub fn tag(&self) -> Option<&'static str> {
        match self {
            Self::VueTs | Self::ReactTs => Some("TS"),
            _ => None,
        }
    }

    /// Category for grouping in UI.
    #[must_use]
    pub fn category(&self) -> SnippetCategory {
        match self {
            Self::Svg | Self::SvgSymbol | Self::Iconify | Self::Jsx => SnippetCategory::Snippets,
            Self::Leptos | Self::Yew | Self::Dioxus => SnippetCategory::Rust,
            Self::Vue
            | Self::VueTs
            | Self::React
            | Self::ReactTs
            | Self::Svelte
            | Self::Qwik
            | Self::Solid
            | Self::Astro => SnippetCategory::Components,
            Self::Url | Self::DataUrl | Self::Base64 | Self::CssBackground => {
                SnippetCategory::Links
            }
        }
    }

    /// All snippet types for iteration.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::Svg,
            Self::SvgSymbol,
            Self::Iconify,
            Self::Jsx,
            Self::Leptos,
            Self::Yew,
            Self::Dioxus,
            Self::Vue,
            Self::VueTs,
            Self::React,
            Self::ReactTs,
            Self::Svelte,
            Self::Qwik,
            Self::Solid,
            Self::Astro,
            Self::Url,
            Self::DataUrl,
            Self::Base64,
            Self::CssBackground,
        ]
    }

    /// Snippet types by category.
    #[must_use]
    pub fn by_category(category: SnippetCategory) -> Vec<Self> {
        Self::all()
            .iter()
            .filter(|s| s.category() == category)
            .copied()
            .collect()
    }
}

/// Convert icon name to `PascalCase` component name.
///
/// # Example
/// ```
/// use rust_icons_core::snippets::to_component_name;
/// assert_eq!(to_component_name("arrow-left"), "ArrowLeft");
/// assert_eq!(to_component_name("mdi:home"), "MdiHome");
/// ```
#[must_use]
pub fn to_component_name(icon: &str) -> String {
    icon.split([':', '-', '_'])
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Clean SVG by keeping only essential attributes.
#[must_use]
pub fn clean_svg(svg: &str) -> String {
    // Simple regex-free cleanup: keep viewBox, xmlns
    svg.to_string()
}

/// Generate a snippet for the given type.
#[must_use]
pub fn generate(icon: &ResolvedIcon, snippet_type: SnippetType) -> String {
    let svg = crate::svg::build_svg(icon);
    let component_name = to_component_name(&format!("{}:{}", icon.prefix, icon.name));
    let icon_id = format!("{}:{}", icon.prefix, icon.name);

    match snippet_type {
        SnippetType::Svg => svg,

        SnippetType::SvgSymbol => format!(
            r#"<symbol id="{}" viewBox="0 0 {} {}">{}</symbol>"#,
            icon_id, icon.width, icon.height, icon.body
        ),

        SnippetType::Iconify => format!(r#"<span class="iconify" data-icon="{icon_id}"></span>"#),

        SnippetType::Jsx => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width={{size}} height={{size}} {{...props}}>{}</svg>"#,
            icon.width, icon.height, icon.body
        ),

        // Rust frameworks
        SnippetType::Leptos => generate_leptos(icon, &component_name),
        SnippetType::Yew => generate_yew(icon, &component_name),
        SnippetType::Dioxus => generate_dioxus(icon, &component_name),

        // Web components
        SnippetType::Vue => generate_vue(icon, &component_name, false),
        SnippetType::VueTs => generate_vue(icon, &component_name, true),
        SnippetType::React => generate_react(icon, &component_name, false),
        SnippetType::ReactTs => generate_react(icon, &component_name, true),
        SnippetType::Svelte => generate_svelte(icon),
        SnippetType::Qwik => generate_qwik(icon, &component_name),
        SnippetType::Solid => generate_solid(icon, &component_name),
        SnippetType::Astro => generate_astro(icon),

        // Links
        SnippetType::Url => crate::svg::iconify_svg_url(&icon.prefix, &icon.name),
        SnippetType::DataUrl => svg_to_data_url(&svg),
        SnippetType::Base64 => svg_to_base64(&svg),
        SnippetType::CssBackground => format!(
            "background: url('{}') no-repeat center center / contain;",
            svg_to_data_url(&svg)
        ),
    }
}

// =============================================================================
// Rust Framework Generators
// =============================================================================

fn generate_leptos(icon: &ResolvedIcon, name: &str) -> String {
    format!(
        r#"use leptos::prelude::*;

#[component]
pub fn {name}(
    #[prop(optional)] class: &'static str,
    #[prop(default = 24)] size: u32,
) -> impl IntoView {{
    view! {{
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 {w} {h}"
            width=size
            height=size
            class=class
            fill="currentColor"
        >
            {body}
        </svg>
    }}
}}"#,
        name = name,
        w = icon.width,
        h = icon.height,
        body = icon.body,
    )
}

fn generate_yew(icon: &ResolvedIcon, name: &str) -> String {
    format!(
        r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(24)]
    pub size: u32,
}}

#[function_component]
pub fn {name}(props: &Props) -> Html {{
    html! {{
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 {w} {h}"
            width={{props.size}}
            height={{props.size}}
            class={{props.class.clone()}}
            fill="currentColor"
        >
            {body}
        </svg>
    }}
}}"#,
        name = name,
        w = icon.width,
        h = icon.height,
        body = icon.body,
    )
}

fn generate_dioxus(icon: &ResolvedIcon, name: &str) -> String {
    // Convert SVG body to Dioxus RSX format
    let rsx_body = svg_body_to_dioxus_rsx(&icon.body);

    format!(
        r#"use dioxus::prelude::*;

#[component]
pub fn {name}(
    #[props(default)] class: Option<String>,
    #[props(default = 24)] size: u32,
) -> Element {{
    rsx! {{
        svg {{
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 {w} {h}",
            width: size,
            height: size,
            class,
            fill: "currentColor",
            {rsx_body}
        }}
    }}
}}"#,
        name = name,
        w = icon.width,
        h = icon.height,
        rsx_body = rsx_body,
    )
}

/// Convert SVG inner body to Dioxus RSX syntax.
fn svg_body_to_dioxus_rsx(body: &str) -> String {
    // Simple conversion: replace kebab-case attrs with snake_case
    // and convert self-closing tags to RSX format
    let mut result = body.to_string();

    // Common SVG attribute conversions
    let replacements = [
        ("fill-rule", "fill_rule"),
        ("clip-rule", "clip_rule"),
        ("stroke-width", "stroke_width"),
        ("stroke-linecap", "stroke_linecap"),
        ("stroke-linejoin", "stroke_linejoin"),
        ("stroke-miterlimit", "stroke_miterlimit"),
        ("fill-opacity", "fill_opacity"),
        ("stroke-opacity", "stroke_opacity"),
    ];

    for (from, to) in replacements {
        result = result.replace(from, to);
    }

    result
}

// =============================================================================
// Web Framework Generators
// =============================================================================

fn generate_vue(icon: &ResolvedIcon, _name: &str, typescript: bool) -> String {
    let script_tag = if typescript {
        "<script setup lang=\"ts\">"
    } else {
        "<script setup>"
    };

    format!(
        r#"<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 {w} {h}"
    :width="size"
    :height="size"
    :class="className"
    fill="currentColor"
  >
    {body}
  </svg>
</template>

{script_tag}
defineProps<{{
  size?: number
  className?: string
}}>()
</script>
"#,
        w = icon.width,
        h = icon.height,
        body = icon.body,
        script_tag = script_tag,
    )
}

fn generate_react(icon: &ResolvedIcon, name: &str, typescript: bool) -> String {
    let props_type = if typescript {
        ": React.SVGProps<SVGSVGElement>"
    } else {
        ""
    };

    let import = if typescript {
        "import React from 'react';\n\n"
    } else {
        ""
    };

    // Convert SVG body attributes to JSX (camelCase)
    let jsx_body = svg_body_to_jsx(&icon.body);

    format!(
        r#"{import}export function {name}(props{props_type}) {{
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 {w} {h}"
      width={{24}}
      height={{24}}
      fill="currentColor"
      {{...props}}
    >
      {jsx_body}
    </svg>
  );
}}

export default {name};
"#,
        import = import,
        name = name,
        props_type = props_type,
        w = icon.width,
        h = icon.height,
        jsx_body = jsx_body,
    )
}

fn generate_svelte(icon: &ResolvedIcon) -> String {
    format!(
        r#"<script>
  export let size = 24;
  export let className = "";
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 {w} {h}"
  width={{size}}
  height={{size}}
  class={{className}}
  fill="currentColor"
  {{...$$restProps}}
>
  {body}
</svg>
"#,
        w = icon.width,
        h = icon.height,
        body = icon.body,
    )
}

fn generate_qwik(icon: &ResolvedIcon, name: &str) -> String {
    let jsx_body = svg_body_to_jsx(&icon.body);

    format!(
        r#"import type {{ QwikIntrinsicElements }} from '@builder.io/qwik';

export function {name}(props: QwikIntrinsicElements['svg']) {{
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 {w} {h}"
      width={{24}}
      height={{24}}
      fill="currentColor"
      {{...props}}
    >
      {jsx_body}
    </svg>
  );
}}

export default {name};
"#,
        name = name,
        w = icon.width,
        h = icon.height,
        jsx_body = jsx_body,
    )
}

fn generate_solid(icon: &ResolvedIcon, name: &str) -> String {
    let jsx_body = svg_body_to_jsx(&icon.body);

    format!(
        r#"import type {{ JSX }} from 'solid-js';

export function {name}(props: JSX.IntrinsicElements['svg']) {{
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 {w} {h}"
      width={{24}}
      height={{24}}
      fill="currentColor"
      {{...props}}
    >
      {jsx_body}
    </svg>
  );
}}

export default {name};
"#,
        name = name,
        w = icon.width,
        h = icon.height,
        jsx_body = jsx_body,
    )
}

fn generate_astro(icon: &ResolvedIcon) -> String {
    format!(
        r#"---
const props = Astro.props;
---

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 {w} {h}"
  width="24"
  height="24"
  fill="currentColor"
  {{...props}}
>
  {body}
</svg>
"#,
        w = icon.width,
        h = icon.height,
        body = icon.body,
    )
}

/// Convert SVG body attributes to JSX camelCase.
fn svg_body_to_jsx(body: &str) -> String {
    let mut result = body.to_string();

    let replacements = [
        ("fill-rule", "fillRule"),
        ("clip-rule", "clipRule"),
        ("stroke-width", "strokeWidth"),
        ("stroke-linecap", "strokeLinecap"),
        ("stroke-linejoin", "strokeLinejoin"),
        ("stroke-miterlimit", "strokeMiterlimit"),
        ("fill-opacity", "fillOpacity"),
        ("stroke-opacity", "strokeOpacity"),
        ("clip-path", "clipPath"),
        ("xlink:href", "xlinkHref"),
    ];

    for (from, to) in replacements {
        result = result.replace(from, to);
    }

    result
}

// =============================================================================
// Data URL Generators
// =============================================================================

/// Encode SVG to Base64.
#[must_use]
pub fn svg_to_base64(svg: &str) -> String {
    base64_encode(svg.as_bytes())
}

/// Simple base64 encoding (no external deps).
fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = String::new();
    let mut i = 0;

    while i < input.len() {
        let b0 = input[i];
        let b1 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        output.push(ALPHABET[(b0 >> 2) as usize] as char);
        output.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);

        if i + 1 < input.len() {
            output.push(ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            output.push('=');
        }

        if i + 2 < input.len() {
            output.push(ALPHABET[(b2 & 0x3f) as usize] as char);
        } else {
            output.push('=');
        }

        i += 3;
    }

    output
}

/// Encode SVG to data URL (chooses shorter of base64 or URL-encoded).
#[must_use]
pub fn svg_to_data_url(svg: &str) -> String {
    let base64 = format!("data:image/svg+xml;base64,{}", svg_to_base64(svg));
    let url_encoded = format!("data:image/svg+xml,{}", encode_svg_for_css(svg));

    if base64.len() < url_encoded.len() {
        base64
    } else {
        url_encoded
    }
}

/// Encode SVG for use in CSS `url()`.
fn encode_svg_for_css(svg: &str) -> String {
    svg.replace('#', "%23")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('"', "'")
        .replace('\n', " ")
        .replace('\r', "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ResolvedIcon;

    fn test_icon() -> ResolvedIcon {
        ResolvedIcon {
            prefix: "mdi".into(),
            name: "arrow-left".into(),
            body: r#"<path d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/>"#
                .into(),
            width: 24,
            height: 24,
        }
    }

    #[test]
    fn to_component_name_works() {
        assert_eq!(to_component_name("arrow-left"), "ArrowLeft");
        assert_eq!(to_component_name("mdi:home"), "MdiHome");
        assert_eq!(to_component_name("foo_bar_baz"), "FooBarBaz");
    }

    #[test]
    fn generate_svg() {
        let icon = test_icon();
        let svg = generate(&icon, SnippetType::Svg);
        assert!(svg.contains("viewBox=\"0 0 24 24\""));
        assert!(svg.contains("<path"));
    }

    #[test]
    fn generate_leptos_component() {
        let icon = test_icon();
        let code = generate(&icon, SnippetType::Leptos);
        assert!(code.contains("#[component]"));
        assert!(code.contains("pub fn MdiArrowLeft"));
        assert!(code.contains("view!"));
    }

    #[test]
    fn generate_yew_component() {
        let icon = test_icon();
        let code = generate(&icon, SnippetType::Yew);
        assert!(code.contains("#[function_component]"));
        assert!(code.contains("pub fn MdiArrowLeft"));
        assert!(code.contains("html!"));
    }

    #[test]
    fn generate_dioxus_component() {
        let icon = test_icon();
        let code = generate(&icon, SnippetType::Dioxus);
        assert!(code.contains("#[component]"));
        assert!(code.contains("pub fn MdiArrowLeft"));
        assert!(code.contains("rsx!"));
    }

    #[test]
    fn generate_data_url() {
        let icon = test_icon();
        let url = generate(&icon, SnippetType::DataUrl);
        assert!(url.starts_with("data:image/svg+xml"));
    }

    #[test]
    fn generate_base64() {
        let icon = test_icon();
        let b64 = generate(&icon, SnippetType::Base64);
        assert!(!b64.is_empty());
        assert!(!b64.contains("data:"));
    }
}
