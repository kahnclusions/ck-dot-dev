use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ck-dot-dev.css"/>

        // sets the document title
        <Title text="<ck.dev />"/>

        <Body class="bg-base text-text" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Routes>
                <Route path="" view=crate::routes::layout::Layout>
                    <Route path="" view=crate::routes::home::HomePage />
                    <Route path="/portfolio" view=crate::routes::portfolio::PortfolioPage />
                    <Route path="/contact" view=crate::routes::contact::ContactPage />
                </Route>
            </Routes>
        </Router>
    }
}
