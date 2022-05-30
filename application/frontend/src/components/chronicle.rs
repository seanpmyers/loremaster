use dioxus::{events::FormEvent, prelude::*};
use crate::utility::constants::API_CHRONICLE_TODAY_URL;
use js_sys::Date;

pub fn Chronicle(context: Scope) -> Element {
    let date = Date::new_0();
    let date_string = Date::to_string(&date);
    context.spawn(async move {
        let response: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
            .get(API_CHRONICLE_TODAY_URL)
            .send()
            .await;

        match response {
            // Parse data from here, such as storing a response token
            Ok(_data) => println!("Registration successful!"),

            //Handle any errors from the fetch here
            Err(_err) => {
                println!(
                    "Registration failed - you need a login server running on localhost:8000."
                )
            }
        }
    });

    context.render(rsx! {
        div { class: "d-flex flex-column",
            h1 { class: "font-bold text-decoration-underline text-center", "{date_string}" }
            div {}
        }
	})
}
