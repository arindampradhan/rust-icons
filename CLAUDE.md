# rust-icons - Claude Code Instructions

## Project Overview

Rust port of [icones](https://github.com/antfu-collective/icones) for Rust component frameworks (Leptos, Yew, Dioxus).

## Code Style (Anthony Fu Inspired)

Follow Anthony Fu's coding philosophy:
- **Simplicity over complexity** - Write code that's easy to understand
- **DX matters** - Developer experience is a feature
- **Convention over configuration** - Sensible defaults, minimal config
- **Composable** - Small, focused, reusable pieces
- **Type-safe** - Leverage Rust's type system fully

## Rust Guidelines

### General
- Use `rustfmt` defaults
- Prefer `clippy` lints at `pedantic` level
- Document public APIs with `///` doc comments
- Use `thiserror` for error types
- Prefer `anyhow` for application errors

### Component Frameworks
- Follow each framework's idiomatic patterns
- Keep components small and focused
- Extract shared logic to `core` crate
- Use feature flags for framework-specific code

### Project Commands

```bash
# Build all
cargo build --workspace

# Test all
cargo test --workspace

# Check formatting
cargo fmt --check

# Lint
cargo clippy --workspace -- -D warnings

# Run Leptos dev server
cd crates/leptos-icons && cargo leptos watch

# Run Yew dev server
cd crates/yew-icons && trunk serve

# Run Dioxus dev server
cd crates/dioxus-icons && dx serve
```

## Architecture Principles

1. **Core crate** - Framework-agnostic icon data, search, utils
2. **Framework crates** - Thin wrappers with framework-specific components
3. **Shared types** - Common types in core, re-exported as needed
4. **WASM-first** - Optimize for browser deployment

## File Organization

- `src/components/` - UI components
- `src/hooks/` - Reactive hooks/signals
- `src/utils/` - Helper functions
- `src/types/` - Type definitions
- `src/api/` - Iconify API client

## Testing

- Unit tests in same file (`#[cfg(test)]`)
- Integration tests in `tests/`
- Use `wasm-bindgen-test` for browser tests
