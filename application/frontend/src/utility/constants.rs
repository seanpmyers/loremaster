pub const API_BASE_URL: &str = "https://localhost:8000";
pub const API_REGISTER_URL: &str = "https://localhost:8000/authentication/register";
pub const API_LOGIN_URL: &str = "https://localhost:8000/authentication/authenticate";
pub const API_CHRONICLE_TODAY_URL: &str = "https://localhost:8000/chronicle/today";
pub const API_PERSON_META_DATA_ROUTE: &str = "person/meta";
pub const API_PERSON_META_UPDATE_ROUTE: &str = "person/update/meta";
pub const API_PERSON_SLEEP_SCHEDULE_UPDATE_ROUTE: &str = "person/update/sleep-schedule";
pub const API_PERSON_SLEEP_SCHEDULE_ROUTE: &str = "person/sleep-schedule";
pub const API_PERSON_EMAIL_ADDRESS_UPDATE_ROUTE: &str = "person/update/email_address";
pub const API_ACTION_NEW_ROUTE: &str = "person/action-new";
pub const API_ACTION_LIST_ROUTE: &str = "person/action-list";
pub const API_FREQUENCY_LIST_ROUTE: &str = "person/frequency-list";
pub const API_GOAL_NEW_ROUTE: &str = "person/goal-new";
pub const API_GOAL_LIST_ROUTE: &str = "person/goal-list";
pub const API_SECURITY_KEY_CHALLENGE_ROUTE: &str = "authentication/security-key-challenge";
pub const HTTP_HEADER_CONTENT_TYPE: &str = "Content-Type";
pub const HTTP_HEADER_CONTENT_TYPE_FORM: &str = "application/x-www-form-urlencoded";
pub const EMAIL_ADDRESS_FIELD: &str = "email_address";
pub const PASSWORD_FIELD: &str = "password";

pub const OK_HTTP_STATUS_CODE: u16 = 200;
pub const ACCEPTED_HTTP_STATUS_CODE: u16 = 202;

pub const JANUARY: &str = "January";
pub const FEBRUARY: &str = "February";
pub const MARCH: &str = "March";
pub const APRIL: &str = "April";
pub const MAY: &str = "May";
pub const JUNE: &str = "June";
pub const JULY: &str = "July";
pub const AUGUST: &str = "August";
pub const SEPTEMBER: &str = "September";
pub const OCTOBER: &str = "October";
pub const NOVEMBER: &str = "November";
pub const DECEMBER: &str = "December";

pub const DAYS_OF_WEEK: &[time::Weekday; 7] = &[
    time::Weekday::Sunday,
    time::Weekday::Monday,
    time::Weekday::Tuesday,
    time::Weekday::Wednesday,
    time::Weekday::Thursday,
    time::Weekday::Friday,
    time::Weekday::Saturday,
];
