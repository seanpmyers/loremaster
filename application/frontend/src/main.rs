use crate::components::navigation::Navigation;
use dioxus::prelude::*;

mod components;
mod data;
mod utility;

fn dioxus_application(context: Scope) -> Element {
    context.render(rsx! {
      div {class: "d-flex flex-column h-100 w-100", Navigation {} }
    })
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    dioxus::web::launch(dioxus_application);
}
