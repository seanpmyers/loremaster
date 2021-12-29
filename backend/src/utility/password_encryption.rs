use anyhow::{
    Result
};
use argon2::{
    Argon2,
    ParamsBuilder, 
    password_hash::{
        rand_core::OsRng,
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier, 
        SaltString
    }
};

use crate::utility::secret_service::{ITERATIONS_FIELD, SITE_SECRET_FIELD, get_secret};

pub struct PasswordEncryptionService {}

pub trait PasswordEncryption {
    fn encrypt_password(input: &str) -> Result<String>;
    fn verify_password(hash: &str, user_input: &str) -> Result<bool>;
}

impl PasswordEncryptionService {

}

impl PasswordEncryption for PasswordEncryptionService {

    fn encrypt_password(credential: &str) -> Result<String> {

        let site_secret: String = get_secret( SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = get_secret( ITERATIONS_FIELD)?.parse::<u32>()?;

        let mut argon2_parameters: ParamsBuilder = argon2::ParamsBuilder::new();
        argon2_parameters.t_cost(iterations_setting).unwrap();

        let argon2 = Argon2::new_with_secret(
            site_secret.as_bytes()
            , argon2::Algorithm::Argon2id
            , argon2::Version::V0x13
            , argon2_parameters.params().unwrap()
        )
        .unwrap();

        let salt: SaltString = SaltString::generate(&mut OsRng);

        let result: String = argon2
            .hash_password(credential.as_bytes(), &salt)
            .unwrap()
            .to_string();

        return Ok(result);
    }

    fn verify_password(hash: &str, credential: &str) -> Result<bool> {
        
        let site_secret: String = get_secret( SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = get_secret( ITERATIONS_FIELD)?.parse::<u32>()?;

        let mut argon2_parameters: ParamsBuilder = argon2::ParamsBuilder::new();
        argon2_parameters.t_cost(iterations_setting).unwrap();

        let argon2 = Argon2::new_with_secret(
            site_secret.as_bytes()
            , argon2::Algorithm::Argon2id
            , argon2::Version::V0x13
            , argon2_parameters.params().unwrap()
        ).unwrap();
        
        let parsed_hash = PasswordHash::new(&hash).unwrap();

        return Ok(argon2.verify_password(&credential.as_bytes(), &parsed_hash).is_ok());
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::password_encryption::{PasswordEncryption, PasswordEncryptionService};

    #[test]
    fn verify_same_key() {
        let hashed_key: String = PasswordEncryptionService::encrypt_password("input").unwrap();
        let hashed_key2: String = PasswordEncryptionService::encrypt_password("input").unwrap();
        let verify_result = PasswordEncryptionService::verify_password(&hashed_key, "input").unwrap();
        assert_ne!(hashed_key, hashed_key2);
        assert_ne!("input", hashed_key);
        assert_eq!(verify_result, true);
    }

    #[test]
    fn verify_different_keys() {
        // The check function should return false if
        let hashed_key: String = PasswordEncryptionService::encrypt_password("pancakes123!").unwrap();
        let verify_result: bool = PasswordEncryptionService::verify_password(&hashed_key, "blueberries325&").unwrap();
        assert_eq!(verify_result, false);
    }

    #[test]
    fn unique_encryption_check() {
        let output1 = PasswordEncryptionService::encrypt_password("input")
            .unwrap();
        let output2 = PasswordEncryptionService::encrypt_password("input")
            .unwrap();
        assert_ne!(output1, output2);
    }
}
