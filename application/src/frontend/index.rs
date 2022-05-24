use std::time::Duration;

use crate::utility::constants::API_REGISTER_URL;
use dioxus::{events::FormEvent, prelude::*};
use log::info;

fn dioxus_application(context: Scope) -> Element {
    let count = use_state(&context, || 0);

    context.spawn({
        let mut count = count.clone();
        async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            count += 1;
        }
    });
    // let onsubmit = move |event: FormEvent| {
    //     info!("Here");
    //     context.spawn(async move {
    //         let response: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
    //             .post(API_REGISTER_URL)
    //             .form(&[
    //                 ("email_address", &event.values["username"]),
    //                 ("password", &event.values["password"]),
    //             ])
    //             .send()
    //             .await;

    //         match response {
    //             // Parse data from here, such as storing a response token
    //             Ok(_data) => println!("Registration successful!"),

    //             //Handle any errors from the fetch here
    //             Err(_err) => {
    //                 println!(
    //                     "Registration failed - you need a login server running on localhost:8000."
    //                 )
    //             }
    //         }
    //     });
    // };

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
                // onsubmit: onsubmit,
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
                button {
                  onclick: move |_| count.set(0),
                  "Reset the count"
                }
                div { "{count}"}
              }
            }
        }
      }
    })
}

pub async fn index() -> String {
    let mut frontend = VirtualDom::new(dioxus_application);
    let _ = frontend.rebuild();
    dioxus::ssr::render_vdom(&frontend)
}

// pub async fn registration() -> String {}
