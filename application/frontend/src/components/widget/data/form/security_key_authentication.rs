use std::ops::Deref;

use js_sys::{Array, Object, Uint8Array};
use perseus::{prelude::spawn_local_scoped, web_log};
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console::log_1, window, AuthenticatorSelectionCriteria, CredentialCreationOptions, Event,
    PublicKeyCredentialCreationOptions, PublicKeyCredentialParameters, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, Window,
};
use webauthn_rs_proto::{CreationChallengeResponse, RegisterPublicKeyCredential};

use crate::{
    components::{
        form::input_validation::InputValidation,
        icon::{FINGERPRINT_SVG_HTML, KEY_2_SVG_HTML},
        state::{message_type::MessageType, validation::Validation, visibility::Visibility},
    },
    utility::{
        constants::{API_BASE_URL, API_WEBAUTHN_START_ROUTE},
        http_service,
    },
};

const SHA_256_SIGNATURE_ALGORITHM: i32 = -7_i32;
const RELAYING_PARTY_ID: &str = "localhost";
const RELAYING_PARTY_NAME: &str = "Loremaster";
const USER_NAME: &str = "username";
const USER_DISPLAY_NAME: &str = "alias";
const USER_ID_STRING: &str = "UZSL85T9AFC";

#[component]
pub fn SecurityKeyAuthentication<G: Html>(context: Scope) -> View<G> {
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
                web_log!("Here");
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

                let form_fields = vec![
                    (
                        String::from("email_address"),
                        email_address.get().as_ref().clone(),
                    ),
                    (String::from("alias"), email_address.get().as_ref().clone()),
                ];

                loading.set(true);
                let query_response = http_service::post_html_form(
                    format!("{}/{}", API_BASE_URL, API_WEBAUTHN_START_ROUTE).as_str(),
                    &form_fields,
                )
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
                let server_challenge: CreationChallengeResponse =
                    serde_json::from_str(&query_response).unwrap();
                // First, convert from our webauthn proto json safe format, into the browser
                // compatible struct, with everything decoded as needed.
                let credential_creation_options: web_sys::CredentialCreationOptions =
                    server_challenge.into();
                // Create a promise that calls the browsers navigator.credentials.create api.
                let promise: js_sys::Promise = window()
                    .unwrap()
                    .navigator()
                    .credentials()
                    .create_with_options(&credential_creation_options)
                    .unwrap();
                match JsFuture::from(promise).await {
                    Ok(value) => {
                        // Convert from the raw js value into the expected PublicKeyCredential
                        let public_key_credential: web_sys::PublicKeyCredential =
                            web_sys::PublicKeyCredential::from(value);
                        // Serialize the web_sys::pkc into the webauthn proto version, ready to
                        // handle/transmit.
                        log_1(&JsValue::from(&public_key_credential));
                        let register_public_key_credential: RegisterPublicKeyCredential =
                            RegisterPublicKeyCredential::from(public_key_credential);
                        // start the fetch routine to post to the server
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
                span() { "Register Security Key" }
            }
        }

        button(class="filled-button", disabled=true) {
            div(class="filled-button-icon", dangerously_set_inner_html=FINGERPRINT_SVG_HTML) {}
            span() { "Register Fingerprint" }
        }
    }
}
