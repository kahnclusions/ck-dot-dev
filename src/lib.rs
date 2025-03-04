pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod routes;
pub mod ui;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
    // leptos::mount_to_body(App);
}
