pub mod database;
pub mod files;

pub const FAILED_LOGIN_MESSAGE: &str =
    "Unable to verify your identity with the credentials you've provided.";
pub const SUCCESSFUL_LOGIN_MESSAGE: &str = "User authenticated successfully!";

pub const REGISTRATION_SUCCESS_MESSAGE: &str = "Account created successfully!";
pub const REGISTRATION_FAILURE_MESSAGE: &str = "Failed to register with the given credentials.";

pub const SYCAMORE_BODY: &str = "%sycamore.body";
