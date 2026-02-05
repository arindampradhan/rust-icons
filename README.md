# rust-icons

A Rust port of [icones](https://github.com/antfu-collective/icones) - an icon explorer with instant search, powered by [Iconify](https://iconify.design/).

## Goal

Port the icones application to Rust-based component systems:

- **[Leptos](https://leptos.dev/)** - Full-stack, fine-grained reactive web framework
- **[Yew](https://yew.rs/)** - Rust/Wasm framework for building client web apps
- **[Dioxus](https://dioxuslabs.com/)** - Fullstack GUI library for Rust

## Inspiration

This project is inspired by [Anthony Fu](https://antfu.me/)'s work on icones and the broader Iconify ecosystem.

## Features (Planned)

- [ ] Icon search and browsing
- [ ] Icon collections support
- [ ] Copy as various formats (SVG, component, etc.)
- [ ] Dark mode support
- [ ] Offline support
- [ ] Multi-framework component output

## Project Structure

```
rust-icons/
├── crates/
│   ├── core/           # Shared core logic
│   ├── leptos-icons/   # Leptos implementation
│   ├── yew-icons/      # Yew implementation
│   └── dioxus-icons/   # Dioxus implementation
├── .claude/            # Claude Code configuration
└── .opencode/          # OpenCode configuration
```

## License

MIT
