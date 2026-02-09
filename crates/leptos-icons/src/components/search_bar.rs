use leptos::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const DEBOUNCE_MS: i32 = 250;

fn cancel_timer(handle: &AtomicI32) {
    let old = handle.swap(0, Ordering::Relaxed);
    if old != 0 {
        if let Some(w) = web_sys::window() {
            w.clear_timeout_with_handle(old);
        }
    }
}

#[component]
pub fn SearchBar(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(default = "Search...")] placeholder: &'static str,
) -> impl IntoView {
    let (raw_input, set_raw_input) = signal(value.get_untracked());
    let timer_id = Arc::new(AtomicI32::new(0));

    // Keep raw_input in sync when value is cleared externally
    Effect::new(move || {
        let v = value.get();
        if v.is_empty() && !raw_input.get_untracked().is_empty() {
            set_raw_input.set(String::new());
        }
    });

    let timer_for_input = Arc::clone(&timer_id);
    let on_input = move |new_val: String| {
        set_raw_input.set(new_val.clone());
        cancel_timer(&timer_for_input);

        let cb = Closure::once_into_js(move || {
            set_value.set(new_val);
        });
        let id = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                DEBOUNCE_MS,
            )
            .unwrap_or(0);
        timer_for_input.store(id, Ordering::Relaxed);
    };

    let timer_for_clear = Arc::clone(&timer_id);
    let on_clear = move |_| {
        cancel_timer(&timer_for_clear);
        set_raw_input.set(String::new());
        set_value.set(String::new());
    };

    view! {
        <div class="search-bar">
            <input
                type="text"
                placeholder=placeholder
                prop:value=move || raw_input.get()
                on:input:target=move |ev| {
                    on_input(ev.target().value());
                }
            />
            <Show when=move || !raw_input.get().is_empty()>
                <button class="clear-btn" on:click=on_clear.clone()>
                    "\u{00d7}"
                </button>
            </Show>
        </div>
    }
}
