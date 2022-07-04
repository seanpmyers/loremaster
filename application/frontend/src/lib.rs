use data::credentials::Credentials;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

pub mod data;

fn submit_registration(credentials: &Credentials) -> bool {
    let request_body: Vec<u8> = format!(
        "email_address={}\n&password={}",
        &credentials.email_address, &credentials.password
    )
    .into_bytes();
    let mut request: ehttp::Request =
        ehttp::Request::post(&format!("http://localhost:8000/register"), request_body);
    request.headers.insert(
        String::from("Content-Type"),
        String::from("application/x-www-form-urlencoded"),
    );
    let mut result: bool = false;
    ehttp::fetch(request, move |fetch_result| match fetch_result {
        Ok(_response) => result = true,
        Err(_error) => result = false,
    });
    result
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
        let credentials = Credentials {
            email_address: email_address.get().as_ref().to_string(),
            password: password.get().as_ref().to_string(),
        };
        let response: bool = submit_registration(&credentials);

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
