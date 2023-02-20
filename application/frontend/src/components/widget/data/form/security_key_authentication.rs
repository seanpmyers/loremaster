use std::ops::Deref;

use js_sys::{Array, Object, Uint8Array};
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, AuthenticatorSelectionCriteria, CredentialCreationOptions, Event, Navigator,
    PublicKeyCredentialCreationOptions, PublicKeyCredentialParameters, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, Window,
};

const SHA_256_SIGNATURE_ALGORITHM: i32 = -7_i32;
const RELAYING_PARTY_ID: &str = "localhost";
const RELAYING_PARTY_NAME: &str = "Loremaster";
const USER_NAME: &str = "username";
const USER_DISPLAY_NAME: &str = "alias";
const USER_ID_STRING: &str = "UZSL85T9AFC";

#[component(SecurityKeyAuthentication<G>)]
pub fn security_key_authentication() -> View<G> {
    let register_handler = move |event: Event| {
        event.prevent_default();
        if G::IS_BROWSER {
            perseus::spawn_local(cloned!( => async move {
                let challenge: Object = Uint8Array::new_with_length(32_u32).deref().to_owned();
                let user_id: Uint8Array = Uint8Array::from(USER_ID_STRING.as_bytes());

                let public_key_credential_parameters: PublicKeyCredentialParameters = PublicKeyCredentialParameters::new(SHA_256_SIGNATURE_ALGORITHM, web_sys::PublicKeyCredentialType::PublicKey);

                let mut relaying_party: PublicKeyCredentialRpEntity = PublicKeyCredentialRpEntity::new(RELAYING_PARTY_NAME);
                relaying_party.id(RELAYING_PARTY_ID);

                let public_key_credential_parameters_array: Array = Array::new_with_length(1);
                public_key_credential_parameters_array.set(0, JsValue::from(public_key_credential_parameters));

                let user: PublicKeyCredentialUserEntity = PublicKeyCredentialUserEntity::new(USER_NAME, USER_DISPLAY_NAME, &user_id.deref());

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
            }));
        }
    };

    view! {
        button(on:click=register_handler) {
            "Register Security Key"
        }
    }
}
