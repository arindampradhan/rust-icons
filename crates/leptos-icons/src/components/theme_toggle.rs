use leptos::prelude::*;

/// Reads the current theme from localStorage or system preference.
/// Returns `true` if dark mode should be active.
fn is_dark_active() -> bool {
    let window = web_sys::window().expect("no window");
    let storage = window.local_storage().ok().flatten();

    match storage.and_then(|s| s.get_item("theme").ok().flatten()) {
        Some(theme) => theme == "dark",
        None => window
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .is_some_and(|mq| mq.matches()),
    }
}

/// Applies or removes the `dark` class on `<html>` and persists choice.
fn apply_theme(dark: bool) {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");

    let html = document.document_element().expect("no <html>");
    let class_list = html.class_list();

    if dark {
        let _ = class_list.add_1("dark");
    } else {
        let _ = class_list.remove_1("dark");
    }

    // Persist to localStorage
    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
        let _ = storage.set_item("theme", if dark { "dark" } else { "light" });
    }
}

/// Theme toggle button — sun icon in dark mode, moon icon in light mode.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = signal(is_dark_active());

    let toggle = move |_: web_sys::MouseEvent| {
        let next = !is_dark.get_untracked();
        set_is_dark.set(next);
        apply_theme(next);
    };

    // Sun icon (shown in dark mode — click to go light)
    let sun_svg = view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"
             fill="none" stroke="currentColor" stroke-width="2"
             stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="5"></circle>
            <line x1="12" y1="1" x2="12" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="23"></line>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
            <line x1="1" y1="12" x2="3" y2="12"></line>
            <line x1="21" y1="12" x2="23" y2="12"></line>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
        </svg>
    };

    // Moon icon (shown in light mode — click to go dark)
    let moon_svg = view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"
             fill="none" stroke="currentColor" stroke-width="2"
             stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
    };

    view! {
        <button
            class="action-btn"
            title=move || if is_dark.get() { "Switch to Morning Edition" } else { "Switch to Evening Edition" }
            on:click=toggle
        >
            <Show when=move || is_dark.get() fallback=move || moon_svg.clone()>
                {sun_svg.clone()}
            </Show>
        </button>
    }
}
