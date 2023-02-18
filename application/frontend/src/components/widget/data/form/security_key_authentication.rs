use std::ops::Deref;

use js_sys::{JsString, Object, Uint8Array};
use perseus::web_log;
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, Navigator, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, Window,
};

#[component(SecurityKeyAuthentication<G>)]
pub fn security_key_authentication() -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!( => async move {
            let challenge: Vec<u8> = vec![2,3,4];
            let pub_key_cred_params: JsValue = JsValue::from_str("{\"alg\": -7, \"type\":\"public-key\"}");
            let mut relaying_party: PublicKeyCredentialRpEntity = PublicKeyCredentialRpEntity::new("Loremaster");
            relaying_party.id("127.0.0.1:8000");
            let user_id: Object = JsString::from("value").deref().to_owned();
            let test: Uint8Array = Uint8Array::from(challenge.as_slice());
            let js_test: &JsValue = test.as_ref();
            web_log!("{}", test.to_string());
            web_sys::console::log_1(&js_test);
            let challenge_object: Object = Uint8Array::new_with_length(32_u32).deref().to_owned();
            let user: PublicKeyCredentialUserEntity = PublicKeyCredentialUserEntity::new("username", "Alias", &user_id);

            let mut public_key_credential_creation_options: PublicKeyCredentialCreationOptions = PublicKeyCredentialCreationOptions::new(
                &challenge_object,
                &pub_key_cred_params,
                &relaying_party,
                &user
            );
            public_key_credential_creation_options.attestation(web_sys::AttestationConveyancePreference::Direct);

            let window_navigator: Navigator = Window::navigator(&window().unwrap());
            let credentials = JsFuture::from(window_navigator.credentials().create().unwrap()).await;
            web_sys::console::log_1(&pub_key_cred_params);
        }));
    }

    view! {}
}
