// use anyhow::{anyhow, Result};
// use surrealdb::{Datastore, Session};

// const SURREAL_DATABASE_FILE_PATH: &str = "file://loremaster.db";
// const SURREAL_NAMESPACE: &str = "loremaster";
// const SURREAL_DATABASE_NAME: &str = "loremaster";

// pub struct SurrealDatabaseHandler {
//     pub session: Session,
//     pub data_store: Datastore,
// }

// impl SurrealDatabaseHandler {
//     pub async fn new() -> Result<Self> {
//         Ok(SurrealDatabaseHandler {
//             session: Session::for_db(SURREAL_NAMESPACE, SURREAL_DATABASE_NAME),
//             data_store: Datastore::new(SURREAL_DATABASE_FILE_PATH)
//                 .await
//                 .map_err(|error| anyhow!("{}", error))?,
//         })
//     }
// }
