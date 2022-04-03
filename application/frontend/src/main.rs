use frontend::App;
use sycamore::prelude::*;

pub mod components;
pub mod lib;
pub mod utility;

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

    sycamore::hydrate_to(|context| view! { context, App() }, &root);
}
