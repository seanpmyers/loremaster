use dioxus::{events::FormEvent, prelude::*};
use dioxus::router::{Link, Route, Router};
use crate::utility::constants::API_AUTHENTICATE_URL;

pub fn Login(context: Scope) -> Element { 
  let onsubmit = move |event: FormEvent| {
    context.spawn(async move {
        let response: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
            .post(API_AUTHENTICATE_URL)
            .form(&[
                ("email_address", &event.values["email_address"]),
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
  div { class: "d-flex justify-content-center p-4",
    form {
      onsubmit: onsubmit,
      prevent_default: "onsubmit",
      h2 { "Login" },
      div { class: "mb-3",
        label { class: "form-label", "Email Address"}
        input { class: "form-control",
          r#type:"text",
          placeholder:"email@example.com",
          id: "email_address",
          name: "email_address"
        }
        div { class: "form-text", "You must register before you can login." }
      }
      div { class: "mb-3",
        label { class: "form-label", "Password"}
        input { class: "form-control",
          placeholder:"",
          r#type:"password",
          id: "password",
          name: "password"
        }
      }
      button { class: "btn text-white acrylic-button-primary", "Submit" }
    }
  }
  
})
} 