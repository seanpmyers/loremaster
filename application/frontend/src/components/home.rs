use dioxus::{events::FormEvent, prelude::*};

pub fn Home(context: Scope) -> Element {
    context.render(rsx! {
            div { class: "d-flex flex-column",
                h1 { class: "font-bold text-decoration-underline text-center", "loremaster" }
                div {}
            }
    })
}
