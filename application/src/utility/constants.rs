pub mod cookie_fields;
pub mod database;
pub mod files;
pub mod unicode;

pub const FAILED_LOGIN_MESSAGE: &str =
    "Unable to verify your identity with the credentials you've provided.";
pub const SUCCESSFUL_LOGIN_MESSAGE: &str = "User authenticated successfully!";

pub const REGISTRATION_SUCCESS_MESSAGE: &str = "Account created successfully!";
pub const INVALID_EMAIL_MESSAGE: &str = "Invalid email address!";
pub const INVALID_PASSWORD_MESSAGE: &str = "Invalid password!";
pub const BLOCKED_EMAIL_MESSAGE: &str =
    "Registration is currently closed as the application is not ready for public users yet. Sorry!";

pub const ENVIRONMENT: &str = "environment";

// pub const FRONTEND_ORIGIN_URL: &str = "http://127.0.0.1:";
pub const LOREMASTER_CONFIGURATION_FILE_PATH: &str = "./Loremaster.toml";
