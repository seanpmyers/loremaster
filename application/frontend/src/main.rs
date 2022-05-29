use crate::components::navigation::Navigation;
use dioxus::router::{Link, Route, Router};
use dioxus::{events::FormEvent, prelude::*};
use std::time::Duration;

mod components;
mod utility;

fn dioxus_application(context: Scope) -> Element {
    context.render(rsx! {
      head { class: "m-4",
        link {
          href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css",
          rel: "stylesheet",
          integrity: "sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3",
          crossorigin: "anonymous"
        }
      }
      body {
        class: "d-flex justify-content-center h-100 w-100",
        div {class: "container-fluid", Navigation {} }
      }
    })
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    dioxus::web::launch(dioxus_application);
}
