pub mod alias_by_id;
pub mod create_person;
pub mod credential_by_email_address;
pub mod get_person_sleep_schedule;
pub mod meta_by_id;
pub mod person_by_email_address;
pub mod update_email_address;
pub mod update_meta_by_id;
pub mod update_person_sleep_schedule;
// #[cfg(test)]
// mod tests {
//     use anyhow::Result;

//     use crate::{
//         data::{
//             entity::person::Person, postgres_handler::PostgresHandler,
//             query::person::create_person::create_person_query,
//         },
//         utility::password_encryption::{PasswordEncryption, PasswordEncryptionService},
//     };

//     use super::person_by_email_address::_person_by_email_address_query;

//     const TEST_EMAIL: &str = "testemail@email.com";
//     const TEST_PASSWORD: &str = "testPassword123!";

//     #[tokio::test]
//     async fn test_create_person() -> Result<()> {
//         let postgres_context: PostgresHandler = PostgresHandler::new().await?;

//         let encrypted_password: String =
//             PasswordEncryptionService::encrypt_password(TEST_PASSWORD)?;

//         let _new_person: Person = create_person_query(
//             &postgres_context.database_pool,
//             &TEST_EMAIL.to_string(),
//             &encrypted_password,
//             None,
//             None,
//         )
//         .await?;

//         return Ok(());
//     }

//     #[tokio::test]
//     async fn test_person_by_email_address_query() -> Result<()> {
//         let postgres_context: PostgresHandler = PostgresHandler::new().await?;

//         let _query_result: Option<Person> = _person_by_email_address_query(
//             &postgres_context.database_pool,
//             &TEST_EMAIL.to_string(),
//         )
//         .await?;

//         return Ok(());
//     }
// }
