use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::web_authentication_challenge::WebAuthenticationRegister;

const QUERY: &str = "
    INSERT INTO 
        public.web_authentication_register (
            id
            , user_name
            , passkey_registration
        )
    VALUES 
        ($1, $2, $3)
    RETURNING
        id
        , user_name
        , passkey_registration
;";

pub async fn add_web_authentication_register_query(
    database_connection: &PgPool,
    web_authentication_register: &WebAuthenticationRegister,
) -> Result<WebAuthenticationRegister> {
    let query_result: WebAuthenticationRegister = query_as::<_, WebAuthenticationRegister>(QUERY)
        .bind(web_authentication_register.id)
        .bind(&web_authentication_register.user_name)
        .bind(&web_authentication_register.passkey)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
