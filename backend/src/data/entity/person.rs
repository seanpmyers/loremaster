#[derive(Debug)]
pub struct Person {
    pub id : Uuid,
    pub email_address : String,
    pub alias : Option<String>,
}

pub struct PersonCredentials {
    pub id: Uuid,
    pub email_address: String,
    pub password: String
}