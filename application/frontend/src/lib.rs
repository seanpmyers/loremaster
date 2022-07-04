use sycamore::prelude::*;
use utility::constants::API_REGISTER_URL;
use utility::http_service::post_html_form;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

pub mod data;
pub mod utility;

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
fn RegistrationForm<G: Html>(context: Scope<'_>) -> View<G> {
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

#[component]
pub fn App<G: Html>(context: Scope) -> View<G> {
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
        RegistrationForm {}
    }
}
