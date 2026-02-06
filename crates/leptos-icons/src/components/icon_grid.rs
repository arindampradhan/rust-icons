use leptos::prelude::*;
use rust_icons_core::svg::iconify_img_url;

const PAGE_SIZE: usize = 200;

#[component]
pub fn IconGrid(
    prefix: String,
    icons: Signal<Vec<String>>,
    on_select: Callback<String>,
) -> impl IntoView {
    let (visible_count, set_visible_count) = signal(PAGE_SIZE);

    // Reset visible count when icons change (e.g. new search)
    Effect::new(move || {
        let _ = icons.get();
        set_visible_count.set(PAGE_SIZE);
    });

    let visible_icons = move || {
        let all = icons.get();
        let limit = visible_count.get().min(all.len());
        all[..limit].to_vec()
    };

    let has_more = move || icons.get().len() > visible_count.get();
    let total = move || icons.get().len();
    let prefix_clone = prefix.clone();

    view! {
        <div class="icon-grid">
            <For
                each=move || visible_icons()
                key=|name| name.clone()
                let:icon_name
            >
                {
                    let prefix = prefix_clone.clone();
                    let icon_name_click = icon_name.clone();
                    let icon_name_display = icon_name.clone();
                    let on_select = on_select;
                    let url = iconify_img_url(&prefix, &icon_name);
                    view! {
                        <div
                            class="icon-cell"
                            data-name=icon_name_display
                            on:click=move |_| on_select.run(icon_name_click.clone())
                        >
                            <img src=url alt="" />
                        </div>
                    }
                }
            </For>
        </div>
        <Show when=has_more>
            <div class="load-more">
                <button on:click=move |_| {
                    set_visible_count.update(|c| *c += PAGE_SIZE);
                }>
                    {move || format!("Load more ({} / {})", visible_count.get().min(total()), total())}
                </button>
            </div>
        </Show>
    }
}
