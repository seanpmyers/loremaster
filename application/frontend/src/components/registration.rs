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

#[component]
pub fn RegistrationForm<G: Html>(context: Scope<'_>) -> View<G> {
    let email_address = create_signal(context, String::new());
    let password = create_signal(context, String::new());

    let handle_subimt = move |event: Event| {
        event.prevent_default();
        if email_address.get().is_empty() || password.get().is_empty() {
            return;
        }

        let response: bool = submit_registration(
            email_address.get().as_ref().to_string(),
            password.get().as_ref().to_string(),
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

    view! { context,
            form(on:submit=handle_subimt) {
                    h1 { "Register"}
                    input(type="email", bind:value=email_address)
                    input(type="password", bind:value=password)
                    button(class="btn") { "Submit" }
            }
    }
}
