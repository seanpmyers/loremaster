use crate::data::chronicle::Chronicle;
use crate::utility::constants::API_CHRONICLE_TODAY_URL;
use dioxus::{events::FormEvent, prelude::*};
use js_sys::Date;

pub fn Demo(context: Scope) -> Element {
    context.render(rsx! {
        div { class: "d-flex flex-grow-1",
            div { class: "d-flex p-4 border-end-0 rounded flex-grow-1",
                h4 { class: "font-bold text-decoration-none text-center", "" }
                div {
                    p { "Good Morning, Sean"}
                }
            }
            div { class: "p-4 background-main vw-20",
                h5 { class: "font-bold text-decoration-none text-center", "Objectives" }
            }
        }
    })
}
