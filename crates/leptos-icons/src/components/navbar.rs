use leptos::prelude::*;

#[component]
#[allow(dead_code)]
pub fn Navbar(
    #[prop(optional, into)] back_href: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <nav class="navbar">
            {back_href.map(|href| view! {
                <a class="back-link" href=href>
                    "\u{2190} Back"
                </a>
            })}
            <h1>{title.unwrap_or_else(|| "Rust Icons".to_string())}</h1>
            {children.map(|c| c())}
        </nav>
    }
}
