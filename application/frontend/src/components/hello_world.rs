use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn HelloWorld<G: Html>(context: Scope<'_>) -> View<G> {
    let name = create_signal(context, String::new());

    let handle_change = move |event: Event| {
        name.set(
            event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        );
    };

    view! { context,
        div {
            h1 {
                "Hello "
                ({if !name.get().is_empty() {
                        view! { context, span { (name.get()) } }
                } else {
                        view! { context, span { "World" } }
                }})
                "!"
            }

            input(placeholder="What is your name?", on:input=handle_change)
        }
    }
}
