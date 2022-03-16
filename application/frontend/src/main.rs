use frontend::App;
use sycamore::prelude::*;

pub mod components;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#sycamore")
        .unwrap()
        .unwrap();

    sycamore::hydrate_to(|| view! { App() }, &root);
}
