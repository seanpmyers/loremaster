use std::ops::Deref;

use js_sys::{Array, Object, Uint8Array};
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, AuthenticatorSelectionCriteria, CredentialCreationOptions, Event,
    PublicKeyCredentialCreationOptions, PublicKeyCredentialParameters, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, Window,
};

use crate::{
    components::icon::{FINGERPRINT_SVG_HTML, KEY_2_SVG_HTML},
    data::entity::security_key::SecurityKeyChallenge,
    utility::{
        constants::{API_BASE_URL, API_SECURITY_KEY_CHALLENGE_ROUTE},
        http_service,
    },
};

const SHA_256_SIGNATURE_ALGORITHM: i32 = -7_i32;
const RELAYING_PARTY_ID: &str = "localhost";
const RELAYING_PARTY_NAME: &str = "Loremaster";
const USER_NAME: &str = "username";
const USER_DISPLAY_NAME: &str = "alias";
const USER_ID_STRING: &str = "UZSL85T9AFC";

#[component(SecurityKeyAuthentication<G>)]
pub fn security_key_authentication() -> View<G> {
    let loading: Signal<bool> = Signal::new(false);
    let email_address: Signal<String> = Signal::new(String::new());
    let email_address_input: Signal<String> = email_address.clone();
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!( => async move {

        }));
    };

    let register_handler = move |event: Event| {
        event.prevent_default();
        if G::IS_BROWSER {
            perseus::spawn_local(cloned!(loading, email_address => async move {
                if email_address.get().is_empty() { return;}
                loading.set(true);
                let query_response: Option<String> = http_service::get_endpoint(format!("{}/{}",API_BASE_URL,API_SECURITY_KEY_CHALLENGE_ROUTE).as_str(), None).await;
                match query_response {
                    Some(response) => {
                        let server_challenge: SecurityKeyChallenge = serde_json::from_str(&response).unwrap();
                        let challenge: Object = Uint8Array::from(server_challenge.challenge.as_slice()).deref().to_owned();
                        let user_id: Uint8Array = Uint8Array::from(server_challenge.user_id.to_string().as_bytes());

                        let public_key_credential_parameters: PublicKeyCredentialParameters = PublicKeyCredentialParameters::new(SHA_256_SIGNATURE_ALGORITHM, web_sys::PublicKeyCredentialType::PublicKey);

                        let mut relaying_party: PublicKeyCredentialRpEntity = PublicKeyCredentialRpEntity::new(server_challenge.relaying_party.as_str());
                        relaying_party.id(&server_challenge.relaying_party_id);

                        let public_key_credential_parameters_array: Array = Array::new_with_length(1);
                        public_key_credential_parameters_array.set(0, JsValue::from(public_key_credential_parameters));

                        let user: PublicKeyCredentialUserEntity = PublicKeyCredentialUserEntity::new(email_address.get().as_ref(), USER_DISPLAY_NAME, &user_id.deref());

                        let mut public_key_credential_creation_options: PublicKeyCredentialCreationOptions = PublicKeyCredentialCreationOptions::new(
                            &challenge,
                            &JsValue::from(public_key_credential_parameters_array),
                            &relaying_party,
                            &user
                        );
                        public_key_credential_creation_options.attestation(web_sys::AttestationConveyancePreference::Direct);

                        let mut authentication_selection_criteria: AuthenticatorSelectionCriteria = AuthenticatorSelectionCriteria::new();
                        authentication_selection_criteria.authenticator_attachment(web_sys::AuthenticatorAttachment::CrossPlatform);
                        public_key_credential_creation_options.authenticator_selection(&authentication_selection_criteria);

                        let mut credential_creation_options: CredentialCreationOptions = CredentialCreationOptions::new();
                        credential_creation_options.public_key(&public_key_credential_creation_options);

                        web_sys::console::log_1(&JsValue::from(public_key_credential_creation_options));

                        let credentials_result = JsFuture::from(Window::navigator(&window().unwrap()).credentials().create_with_options(&credential_creation_options).unwrap()).await;

                        match credentials_result {
                            Ok(result) => web_sys::console::log_1(&result),
                            Err(error) => web_sys::console::log_1(&error),
                        }

                        // let test = window_navigator.credentials().create_with_options(&credential_creation_options).unwrap();
                    },
                    None => {},
                }
                loading.set(false);
            }));
        }
    };

    view! {
        form() {
            div(class="input-row") {
                label() { "Email Address" }
                input(
                    type="email",
                    bind:value=email_address_input,
                    placeholder = "Enter your email address"
                ) {}
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
