use dioxus::{events::FormEvent, prelude::*};
use std::time::Duration;

mod utility;

use crate::utility::constants::API_REGISTER_URL;

fn dioxus_application(context: Scope) -> Element {
    let onsubmit = move |event: FormEvent| {
        context.spawn(async move {
            let response: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
                .crossorigin
                .post(API_REGISTER_URL)
                .form(&[
                    ("email_address", &event.values["username"]),
                    ("password", &event.values["password"]),
                ])
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
    };

    context.render(rsx! {
      head { class: "m-4",
        link {
          href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css",
          rel: "stylesheet",
          integrity: "sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3",
          crossorigin: "anonymous"
        }
      }
      body {
        class: "d-flex justify-content-center h-100 w-100",
        div { class: "d-flex flex-column",
          h1 { class: "font-bold text-decoration-underline text-center", "loremaster" }
            div {
              form {
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                div {
                  label {"Email Address"}
                  input {
                    r#type:"text",
                    placeholder:"email@example.com",
                    id: "username",
                    name: "username"
                  }
                }
                div {
                  label {"Password"}
                  input {
                    placeholder:"",
                    r#type:"password",
                    id: "password",
                    name: "password"
                  }
                }
                div {
                  button {
                    "Submit"
                  }
                }
              }
            }
        }
      }
    })
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    dioxus::web::launch(dioxus_application);
}
