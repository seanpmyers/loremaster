use dioxus::{events::FormEvent, prelude::*};

pub fn Home(context: Scope) -> Element {
    context.render(rsx! {
            div { class: "d-flex flex-column p-4",
                h1 { class: "font-bold text-decoration-none text-center", "Loremaster" }
                div {}
            }
    })
}
