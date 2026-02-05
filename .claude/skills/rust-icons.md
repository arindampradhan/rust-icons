# rust-icons

Project-specific skill for the rust-icons codebase.

## Quick Commands

### Build & Test
```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

### Framework-Specific

**Leptos:**
```bash
cd crates/leptos-icons && cargo leptos watch
```

**Yew:**
```bash
cd crates/yew-icons && trunk serve
```

**Dioxus:**
```bash
cd crates/dioxus-icons && dx serve
```

## Architecture

```
crates/
├── core/           # Shared: IconData, Search, API client
├── leptos-icons/   # Leptos components + app
├── yew-icons/      # Yew components + app
└── dioxus-icons/   # Dioxus components + app
```

## Core Types

```rust
// Icon representation
pub struct Icon {
    pub name: String,
    pub collection: String,
    pub svg: String,
    pub width: u32,
    pub height: u32,
}

// Collection metadata
pub struct Collection {
    pub id: String,
    pub name: String,
    pub total: usize,
    pub author: Option<String>,
    pub license: String,
}
```

## Iconify API

Base URL: `https://api.iconify.design`

Endpoints:
- `GET /collections` - List all collections
- `GET /{prefix}.json` - Get collection icons
- `GET /{prefix}/{name}.svg` - Get single icon SVG

## Component Patterns

### Leptos
```rust
#[component]
pub fn Icon(name: String, #[prop(optional)] size: Option<u32>) -> impl IntoView {
    let size = size.unwrap_or(24);
    view! { <svg width=size height=size>/* ... */</svg> }
}
```

### Yew
```rust
#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    html! { <svg width={props.size} height={props.size}>/* ... */</svg> }
}
```

### Dioxus
```rust
#[component]
pub fn Icon(name: String, size: Option<u32>) -> Element {
    let size = size.unwrap_or(24);
    rsx! { svg { width: size, height: size, /* ... */ } }
}
```
