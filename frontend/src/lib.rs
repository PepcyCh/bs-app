#![recursion_limit = "512"]
#[rustfmt::skip::macros(html)]
mod app;
mod pages;
mod route;
mod utils;

use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_as_body();
}
