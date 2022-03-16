use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, ParamsBuilder, PasswordVerifier,
};

use crate::utility::secret_service::{get_secret, ITERATIONS_FIELD, SITE_SECRET_FIELD};

pub struct PasswordEncryptionService {}

pub trait PasswordEncryption {
    fn encrypt_password(input: &str) -> Result<String>;
    fn verify_password(encrypted_password: &str, user_input: &str) -> Result<bool>;
}

impl PasswordEncryptionService {}

impl PasswordEncryption for PasswordEncryptionService {
    fn encrypt_password(credential: &str) -> Result<String> {
        let site_secret: String = get_secret(SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = get_secret(ITERATIONS_FIELD)?.parse::<u32>()?;
        let mut argon2_parameters: ParamsBuilder = argon2::ParamsBuilder::new();

        argon2_parameters
            .t_cost(iterations_setting)
            .map_err(|error| anyhow!("{}", error))?;

        let argon2 = Argon2::new_with_secret(
            site_secret.as_bytes(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2_parameters.params().unwrap(),
        )
        .map_err(|error| anyhow!("{}", error))?;

        let salt: SaltString = SaltString::generate(&mut OsRng);

        let result: String = argon2
            .hash_password(credential.as_bytes(), &salt)
            .map_err(|error| anyhow!("{}", error))?
            .to_string();

        Ok(result)
    }

    fn verify_password(encrypted_password: &str, credential: &str) -> Result<bool> {
        let site_secret: String = get_secret(SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = get_secret(ITERATIONS_FIELD)?.parse::<u32>()?;

        let mut argon2_parameters: ParamsBuilder = argon2::ParamsBuilder::new();
        argon2_parameters.t_cost(iterations_setting).unwrap();

        let argon2 = Argon2::new_with_secret(
            site_secret.as_bytes(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2_parameters.params().unwrap(),
        )
        .map_err(|error| anyhow!("{}", error))?;

        let parsed_hash = PasswordHash::new(encrypted_password).unwrap();

        Ok(argon2
            .verify_password(credential.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::password_encryption::{PasswordEncryption, PasswordEncryptionService};
    use anyhow::Result;

    #[test]
    fn verify_same_key() -> Result<()> {
        let encrypted_key: String = PasswordEncryptionService::encrypt_password("input")?;
        let encrypted_key2: String = PasswordEncryptionService::encrypt_password("input")?;
        let verify_result: bool =
            PasswordEncryptionService::verify_password(&encrypted_key, "input")?;
        assert_ne!(encrypted_key, encrypted_key2);
        assert_ne!("input", encrypted_key);
        assert_eq!(verify_result, true);
        Ok(())
    }

    #[test]
    fn verify_different_keys() -> Result<()> {
        // The check function should return false if
        let verify_result: bool = PasswordEncryptionService::verify_password(
            &PasswordEncryptionService::encrypt_password("pancakes123!")?,
            "blueberries325&",
        )?;
        assert_eq!(verify_result, false);
        Ok(())
    }

    #[test]
    fn unique_encryption_check() -> Result<()> {
        assert_ne!(
            PasswordEncryptionService::encrypt_password("input")?,
            PasswordEncryptionService::encrypt_password("input")?
        );
        Ok(())
    }
}
