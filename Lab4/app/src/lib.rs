pub mod app;

#[cfg(feature = "ssr")]
pub mod db;


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount_to_body(App);
}