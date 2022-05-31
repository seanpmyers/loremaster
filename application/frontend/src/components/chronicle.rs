use crate::data::chronicle::Chronicle;
use crate::utility::constants::API_CHRONICLE_TODAY_URL;
use dioxus::{events::FormEvent, prelude::*};
use js_sys::Date;

pub fn Chronicle(context: Scope) -> Element {
    let date = Date::new_0();
    let date_string = Date::to_string(&date);

    let possible_response = use_future(&context, (), |_| async move {
        reqwest::Client::new()
        .get(API_CHRONICLE_TODAY_URL)
        .send()
        .await
        .unwrap()
        .json::<Chronicle>()
        .await
    });


    let name: String = match possible_response.value() {
        Some(Ok(val)) => val.person_id.clone(),
        Some(Err(error)) => "Stranger".to_string(),
        None => "Stranger".to_string(),
    };

    context.render(rsx! {
        div { class: "d-flex flex-grow-1",
            div { class: "d-flex p-4 border-end-0 rounded flex-grow-1",
                h4 { class: "font-bold text-decoration-none text-center", "{date_string}" }
                div {
                    p { "Good Morning, {name}"}
                }
            }
            div { class: "p-4 background-main vw-20",
                h5 { class: "font-bold text-decoration-none text-center", "Objectives" }
            }
        }
    })
}
