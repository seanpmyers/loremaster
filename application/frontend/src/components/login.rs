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
  div { class: "d-flex flex-column",
    h2 { "Login" },
    form {
      onsubmit: onsubmit,
      prevent_default: "onsubmit",
      div { class: "mb-3",
        label { class: "form-label", "Email Address"}
        input { class: "form-control",
          r#type:"text",
          placeholder:"email@example.com",
          id: "email_address",
          name: "email_address"
        }
        div { class: "form-text", "Emails are encrypted before being stored in the database." }
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
      button { class: "btn btn-primary", "Submit" }
    }
  }
  
})
} 