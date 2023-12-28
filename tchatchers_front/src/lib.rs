pub(crate) mod components;
pub(crate) mod router;
pub(crate) mod utils;
pub mod app;

use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}