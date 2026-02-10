use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages::collection::CollectionPage;
use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="paper-container">
            <Router>
                <Routes fallback=|| view! { <div class="loading">"Page not found."</div> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/collection/:id") view=CollectionPage />
                </Routes>
            </Router>
        </div>
    }
}
