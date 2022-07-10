pub mod cookie_fields;
pub mod database;
pub mod files;

pub const FAILED_LOGIN_MESSAGE: &str =
    "Unable to verify your identity with the credentials you've provided.";
pub const SUCCESSFUL_LOGIN_MESSAGE: &str = "User authenticated successfully!";

pub const REGISTRATION_SUCCESS_MESSAGE: &str = "Account created successfully!";

pub const PROFILE: &str = "LOREMASTER_PROFILE";
pub const LOCAL_DEBUG: &str = "LOCAL";
pub const _API_BASE_URL: &str = "http://localhost:8000";
pub const _API_REGISTER_URL: &str = "http://localhost:8000/register";
