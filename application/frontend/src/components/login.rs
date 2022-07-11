use sycamore::prelude::*;
use web_sys::Event;

use crate::utility::{constants::API_AUTHENTICATE_URL, http_service::post_html_form};

fn submit_registration(email_address: String, password: String) -> bool {
    post_html_form(
        &API_AUTHENTICATE_URL.to_string(),
        &vec![
            (String::from("email_address"), email_address),
            (String::from("password"), password),
        ],
    )
}

#[component(LoginForm<G>)]
pub fn login_form() -> View<G> {
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
        div(class="container background-acrylic-white border-0 rounded p-5 shadow") {
            div(class="row") {
                div(class="col-12") {
                    div(clas="card") {
                        div(class="card-body") {
                            form(on:submit=handle_subimt) {
                                h1 { "Login"}
                                div(class="mb-3") {
                                    label(class="form-label", for="email") {"Email Address"}
                                    input(class="form-control", type="email", bind:value=email_address)

                                }
                                div(class="mb-3") {
                                    label(class="form-label", for="password") {"Password"}
                                    input(class="form-control", type="password", bind:value=password)
                                    div(class="form-text") {
                                        "We recommend using a password manager like KeePass to generate and store your password."
                                    }
                                }
                                button(class="btn btn-primary") { "Submit" }
                            }
                        }
                    }
                }
            }
        }
    }
}
