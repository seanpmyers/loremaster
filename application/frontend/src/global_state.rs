use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const LOCAL_STORAGE_KEY: &str = "chronilore_loremaster";

#[derive(Clone, Serialize, Deserialize)]
pub enum AuthenticationState {
    Authenticated,
    Anonymous,
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
            user_alias: String::new(),
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

    pub fn detect_state(&self) {
        if let AuthenticationState::Authenticated | AuthenticationState::None =
            *self.authentication_state.get()
        {
            return;
        }

        let storage: web_sys::Storage =
            web_sys::window().unwrap().local_storage().unwrap().unwrap();

        match storage.get(LOCAL_STORAGE_KEY).unwrap() {
            Some(data) => {
                let stored_authentication: BrowserCache = serde_json::from_str(&data).unwrap();
            }
            None => (),
        }
    }

    pub fn update_user_alias(&self, new_alias: &str) {
        self.user_alias.set(new_alias.to_string());
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
        self.user_alias.set(String::new());
    }
}
