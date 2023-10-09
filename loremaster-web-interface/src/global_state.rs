use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utility::http_service;

pub const LOCAL_STORAGE_KEY: &str = "chronilore_loremaster";
pub const DEFAULT_USER_ALIAS: &str = "You";

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum AuthenticationState {
    Anonymous,
    Authenticated,
    None,
}

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "ApplicationStateRx")]
pub struct ApplicationState {
    /// Authentication data accessible to all pages.
    #[rx(nested)]
    pub authentication: UserAuthentication,
}

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "UserAuthenticationRx")]
pub struct UserAuthentication {
    pub authentication_state: AuthenticationState,
    pub user_alias: String,
    pub session_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct BrowserCache {
    pub user_alias: String,
}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[engine_only_fn]
async fn get_build_state() -> ApplicationState {
    ApplicationState {
        authentication: UserAuthentication {
            authentication_state: AuthenticationState::None,
            user_alias: String::from(DEFAULT_USER_ALIAS),
            session_id: Uuid::new_v4(),
        },
    }
}

impl UserAuthenticationRx {
    pub fn to_browser_cache(&self) -> BrowserCache {
        BrowserCache {
            user_alias: String::from(self.user_alias.get().as_str()),
        }
    }

    pub async fn detect_state(&self) {
        if let AuthenticationState::Anonymous = *self.authentication_state.get() {
            return;
        }

        if let AuthenticationState::None = *self.authentication_state.get() {
            //make a request for now to determine if the cookie is alive
            let _ = http_service::get_endpoint("/authentication/check-session", None).await;
            self.authentication_state
                .set(AuthenticationState::Authenticated);
        }

        let storage: web_sys::Storage =
            web_sys::window().unwrap().local_storage().unwrap().unwrap();

        if let Some(cache_json) = storage.get(LOCAL_STORAGE_KEY).unwrap() {
            let stored_authentication: BrowserCache = serde_json::from_str(&cache_json).unwrap();
            self.user_alias.set(stored_authentication.user_alias);
        }
    }

    pub fn update_authentication_state(&self, new_state: AuthenticationState) {
        if *self.authentication_state.get().to_owned() == new_state {
            return;
        }

        self.authentication_state.set(new_state);
        self.update_authentication();
    }

    pub fn update_user_alias(&self, new_alias: &str) {
        if new_alias.eq(self.user_alias.get().as_str()) {
            return;
        }

        self.user_alias.set(new_alias.to_string());
        self.update_authentication();
    }

    pub fn update_authentication(&self) {
        let storage: web_sys::Storage =
            web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let cache: BrowserCache = self.to_browser_cache();
        let serialized_cache = serde_json::to_string(&cache).unwrap();
        storage.set(LOCAL_STORAGE_KEY, &serialized_cache).unwrap();
    }

    pub fn logout(&self) {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.delete(LOCAL_STORAGE_KEY).unwrap();
        self.authentication_state
            .set(AuthenticationState::Anonymous);
        self.user_alias.set(String::from(DEFAULT_USER_ALIAS));
    }
}
