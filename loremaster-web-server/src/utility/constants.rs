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

// pub const FRONTEND_ORIGIN_URL: &str = "http://127.0.0.1:";
pub const LOREMASTER_CONFIGURATION_FILE_PATH: &str = "./Loremaster.ron";
pub const RELAYING_PARTY_ID: &str = "chronilore.day";
pub const DEV_RELAYING_PARTY_ID: &str = "chronilore.day";
pub const QA_RELAYING_PARTY_ID: &str = "chronilore.day";
pub const LOCAL_HOST_RELAYING_PARTY_ID: &str = "localhost";
pub const RELAYING_PARTY: &str = "Loremaster";
