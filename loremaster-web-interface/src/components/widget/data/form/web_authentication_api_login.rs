use gloo_timers::future::TimeoutFuture;
use perseus::{
    prelude::{navigate, spawn_local_scoped},
    reactor::Reactor,
    web_log,
};
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console::log_1, window, Event};
use webauthn_rs_proto::{PublicKeyCredential, RequestChallengeResponse};

use crate::{
    components::{
        form::input_validation::InputValidation,
        icon::KEY_2_SVG_HTML,
        state::{message_type::MessageType, validation::Validation, visibility::Visibility},
    },
    data::entity::{person_meta::PersonMeta, webauthn::PersonPublicKeyCredential},
    global_state::ApplicationStateRx,
    utility::{
        constants::{
            API_BASE_URL, API_PERSON_META_DATA_ROUTE, API_WEBAUTHN_LOGIN_END_ROUTE,
            API_WEBAUTHN_LOGIN_START_ROUTE,
        },
        http_service,
    },
};

#[component]
pub fn WebAuthenticationAPILogin<G: Html>(context: Scope) -> View<G> {
    let ApplicationStateRx { authentication } =
        Reactor::<G>::from_cx(context).get_global_state::<ApplicationStateRx>(context);
    let loading: &Signal<bool> = create_signal(context, false);
    let email_address: &Signal<String> = create_signal(context, String::new());

    let email_address_validation_content: &Signal<String> = create_signal(context, String::new());
    let email_address_validation_visibility: &Signal<Visibility> =
        create_signal(context, Visibility::Hidden);
    let email_address_validity: &Signal<Validation> = create_signal(context, Validation::Valid);
    let email_address_message_type: &Signal<MessageType> =
        create_signal(context, MessageType::Information);

    if G::IS_BROWSER {
        spawn_local_scoped(context, async move {});
    };

    let register_handler = move |event: Event| {
        event.prevent_default();
        if G::IS_BROWSER {
            spawn_local_scoped(context, async move {
                if loading.get().as_ref() == &true {
                    return;
                }

                if email_address.get().is_empty() {
                    email_address_validation_content
                        .set(String::from("Email address cannot be empty."));
                    email_address_validation_visibility.set(Visibility::Visible);
                    email_address_message_type.set(MessageType::Error);
                    email_address_validity.set(Validation::Invalid);
                    loading.set(false);
                    return;
                }

                let form_fields = vec![(
                    String::from("email_address"),
                    email_address.get().as_ref().clone(),
                )];
                web_log!("Here");

                loading.set(true);
                //TODO: Fix error handling
                let query_response = http_service::post_html_form(
                    format!("{}/{}", API_BASE_URL, API_WEBAUTHN_LOGIN_START_ROUTE).as_str(),
                    &form_fields,
                )
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
                let server_challenge: RequestChallengeResponse =
                    serde_json::from_str(&query_response).unwrap();
                // First, convert from our webauthn proto json safe format, into the browser
                // compatible struct, with everything decoded as needed.
                let credential_creation_options: web_sys::CredentialRequestOptions =
                    server_challenge.into();
                // Create a promise that calls the browsers navigator.credentials.create api.
                let promise: js_sys::Promise = window()
                    .unwrap()
                    .navigator()
                    .credentials()
                    .get_with_options(&credential_creation_options)
                    .unwrap();
                match JsFuture::from(promise).await {
                    Ok(value) => {
                        // Convert from the raw js value into the expected PublicKeyCredential
                        let public_key_credential: web_sys::PublicKeyCredential =
                            web_sys::PublicKeyCredential::from(value);
                        // Serialize the web_sys::pkc into the webauthn proto version, ready to
                        // handle/transmit.
                        log_1(&JsValue::from(&public_key_credential));
                        let public_key_credential: PublicKeyCredential =
                            PublicKeyCredential::from(public_key_credential);
                        // start the fetch routine to post to the server
                        let login_input: PersonPublicKeyCredential = PersonPublicKeyCredential {
                            email_address: email_address.get().as_ref().clone(),
                            public_key_credential,
                        };
                        let json = serde_json::to_string(&login_input).unwrap();
                        match http_service::post_json(
                            &format!("{}/{}", API_BASE_URL, API_WEBAUTHN_LOGIN_END_ROUTE),
                            json,
                        )
                        .await
                        {
                            Some(response) => {
                                if response.ok() {
                                    let query_response: Option<String> =
                                        http_service::get_endpoint(
                                            format!(
                                                "{}/{}",
                                                API_BASE_URL, API_PERSON_META_DATA_ROUTE
                                            )
                                            .as_str(),
                                            None,
                                        )
                                        .await;
                                    match query_response {
                                        Some(response) => {
                                            let person_meta_data: PersonMeta =
                                                serde_json::from_str(&response).unwrap();
                                            email_address.set(person_meta_data.email_address);
                                            if let Some(existing_alias) = person_meta_data.alias {
                                                authentication.update_user_alias(&existing_alias);
                                            }
                                        }
                                        None => todo!(),
                                    }
                                    TimeoutFuture::new(4000_u32).await;
                                    navigate("/chronicle/");
                                }
                            }
                            None => todo!(),
                        }
                    }
                    Err(_) => todo!(),
                };
                loading.set(false);
            });
        }
    };

    view! { context,
        form() {
            div(class="input-row") {
                label() { "Email Address" }
                input(
                    type="email",
                    bind:value=email_address,
                    placeholder = "Enter your email address"
                ) {}
                InputValidation(
                    content= email_address_validation_content,
                    visibility= email_address_validation_visibility,
                    validity= email_address_validity,
                    message_type= email_address_message_type)
            }
            button(class="filled-button", on:click=register_handler) {
                div(class="filled-button-icon", dangerously_set_inner_html=KEY_2_SVG_HTML) {}
                span() { "Login with Security Key" }
            }
        }
    }
}
