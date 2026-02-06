use leptos::ev::MouseEvent;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn BottomDrawer(on_close: Callback<()>, children: Children) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Trigger opening animation after mount
    Effect::new(move |_| {
        set_is_open.set(true);

        // Prevent body scroll when drawer is open
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.class_list().add_1("drawer-open");
        }
    });

    let close_drawer = move || {
        set_is_open.set(false);

        // Re-enable body scroll
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.class_list().remove_1("drawer-open");
        }

        // Wait for animation to finish before calling on_close
        if let Some(window) = web_sys::window() {
            let closure = Closure::once(move || {
                on_close.run(());
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                300,
            );
            closure.forget();
        }
    };

    let on_overlay_click = move |_: MouseEvent| {
        close_drawer();
    };

    let stop_propagation = move |ev: MouseEvent| {
        ev.stop_propagation();
    };

    // Add keyboard listener on mount for Escape key
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    close_drawer();
                }
            }) as Box<dyn FnMut(_)>);

            let _ = window
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    view! {
        <div
            class="drawer-overlay"
            class:open=move || is_open.get()
            on:click=on_overlay_click
            tabindex="-1"
        >
            <div class="drawer-content" on:click=stop_propagation>
                <div class="drawer-handle" on:click=move |_| close_drawer()></div>
                {children()}
            </div>
        </div>
    }
}
