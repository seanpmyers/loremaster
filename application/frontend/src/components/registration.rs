use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense};

use crate::utility::constants::API_REGISTER_URL;

#[derive(Serialize, Deserialize, Default, Debug)]
struct RegisterResponse {
    value: String,
}

async fn register_user(
    email_address: String,
    password: String,
) -> Result<RegisterResponse, reqwasm::Error> {
    let response: reqwasm::http::Response = Request::post(&format!(
        "{API_REGISTER_URL}/?email_address={email_address}&password={password}"
    ))
    .send()
    .await?;

    let body: RegisterResponse = response.json::<RegisterResponse>().await?;

    Ok(body)
}

#[component]
pub async fn RegistrationForm<G: Html>(context: Scope<'_>) -> View<G> {
    let email_address: &Signal<String> = create_signal(context, String::new());
    let password: &Signal<String> = create_signal(context, String::new());

    let submit_registration = move |_| {
        spawn_local_scoped(context, async move {
            if email_address.get().is_empty() || password.get().is_empty() {
                return;
            }
            let response = register_user(
                email_address.get().as_ref().clone(),
                password.get().as_ref().clone(),
            )
            .await
            .unwrap_or_default();
        })
    };

    view! { context,
            form {
                div {
                    label {"Email Address"}
                    input(placeholder="you@loremaster.xyz", type="text", bind=email_address.clone())
                }
                div {
                    label {"Password"}
                    input(placeholder="", type="password", bind=password.clone())
                }
                div {
                    button(on:submit=submit_registration)
                }
        }
    }
}

#[component]
pub fn Registration<G: Html>(context: Scope) -> View<G> {
    view! { context,
        div {
            h1 {
                "Register"
            }
            Suspense {
                fallback: view!{ context, "Loading..."},
                RegistrationForm {}
            }

        }
    }
}
