#![recursion_limit = "512"]
#[rustfmt::skip::macros(html)]
mod app;
mod pages;
mod route;
mod utils;

fn main() {
    yew::start_app::<app::App>();
}
