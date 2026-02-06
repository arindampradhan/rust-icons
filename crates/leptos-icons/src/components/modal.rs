use leptos::prelude::*;

#[component]
pub fn Modal(on_close: Callback<()>, children: Children) -> impl IntoView {
    let on_overlay_click = move |_| {
        on_close.run(());
    };

    let stop_propagation = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <div class="modal-overlay" on:click=on_overlay_click>
            <div class="modal-content" on:click=stop_propagation>
                {children()}
            </div>
        </div>
    }
}
