use sycamore::prelude::*;
use web_sys::Event;

use crate::utility::{constants::API_REGISTER_URL, http_service::post_html_form};

fn submit_registration(email_address: String, password: String) -> bool {
    post_html_form(
        &API_REGISTER_URL.to_string(),
        &vec![
            (String::from("email_address"), email_address),
            (String::from("password"), password),
        ],
    )
}

#[component(RegistrationForm<G>)]
pub fn registration_form() -> View<G> {
    let email_address: Signal<String> = Signal::new(String::new());
    let cloned_email_address = email_address.clone();
    let password: Signal<String> = Signal::new(String::new());
    let cloned_password = password.clone();

    let handle_subimt = move |event: Event| {
        event.prevent_default();
        if cloned_email_address.get().is_empty() || cloned_password.get().is_empty() {
            return;
        }

        let response: bool = submit_registration(
            cloned_email_address.get().as_ref().to_string(),
            cloned_password.get().as_ref().to_string(),
        );

        match response {
            // Parse data from here, such as storing a response token
            true => println!("Registration successful!"),

            //Handle any errors from the fetch here
            false => {
                println!("Registration failed - you need a login server running on localhost:8000.")
            }
        }
    };

    view! {
        div {
                form(on:submit=handle_subimt) {
                    h1 { "Register"}
                    input(type="email", bind:value=email_address)
                    input(type="password", bind:value=password)
                    button(class="btn") { "Submit" }
            }
        }
    }
}
