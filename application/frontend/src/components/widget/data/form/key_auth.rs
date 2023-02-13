use js_sys::Object;
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{
    window, Navigator, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, Window,
};

#[component(ValueList<G>)]
pub fn key_auth() -> View<G> {
    if G::IS_BROWSER {
        perseus::spawn_local(cloned!( => async move {
            let challenge: Vec<u8> = vec![];
            let pub_key_cred_params: JsValue = JsValue::from_str("{\"alg\": -7, \"type\":\"public-key\"}");
            let mut relaying_party: PublicKeyCredentialRpEntity = PublicKeyCredentialRpEntity::new("Loremaster");
            relaying_party.id("127.0.0.1:8000");
            let user_id: Object = Object::new();
            let challenge_object: Object = Object::new();
            let user: PublicKeyCredentialUserEntity = PublicKeyCredentialUserEntity::new("username", "Alias", &user_id);
            let public_key_credential_creation_options: PublicKeyCredentialCreationOptions = PublicKeyCredentialCreationOptions::new(&challenge_object, &pub_key_cred_params, &relaying_party, &user);
            let window_navigator: Navigator = Window::navigator(&window().unwrap());
            let credentials = window_navigator.credentials().create();
        }));
    }

    view! {}
}
