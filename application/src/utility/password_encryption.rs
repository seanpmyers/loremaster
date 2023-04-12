use std::time::Instant;

use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, ParamsBuilder, PasswordVerifier,
};
use log::info;

#[derive(Clone)]
pub struct PasswordEncryptionService {
    pub iterations: u32,
    pub site_secret: String,
}

pub trait PasswordEncryption {
    fn new(iterations: u32, site_secret: String) -> Self;
    fn encrypt_password(&self, input: &str) -> Result<String>;
    fn verify_password(&self, encrypted_password: &str, user_input: &str) -> Result<bool>;
}

impl PasswordEncryptionService {}

impl PasswordEncryption for PasswordEncryptionService {
    fn new(iterations: u32, site_secret: String) -> Self {
        PasswordEncryptionService {
            iterations,
            site_secret,
        }
    }

    fn encrypt_password(&self, credential: &str) -> Result<String> {
        let now: Instant = Instant::now();
        let mut argon2_parameters: ParamsBuilder = argon2::ParamsBuilder::new();

        argon2_parameters.t_cost(self.iterations).p_cost(2);

        let argon2 = Argon2::new_with_secret(
            self.site_secret.as_bytes(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2_parameters.build().unwrap(),
        )
        .map_err(|error| anyhow!("{}", error))?;

        let salt: SaltString = SaltString::generate(&mut OsRng);

        let result: String = argon2
            .hash_password(credential.as_bytes(), &salt)
            .map_err(|error| anyhow!("{}", error))?
            .to_string();
        info!(
            "Encrypted password: elapsed: {}ms",
            now.elapsed().as_millis().to_string()
        );
        Ok(result)
    }

    fn verify_password(&self, encrypted_password: &str, credential: &str) -> Result<bool> {
        let now: Instant = Instant::now();
        let mut argon2_parameters: ParamsBuilder = argon2::ParamsBuilder::new();
        argon2_parameters.t_cost(self.iterations).p_cost(2);

        let argon2 = Argon2::new_with_secret(
            self.site_secret.as_bytes(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2_parameters.build().unwrap(),
        )
        .map_err(|error| anyhow!("{}", error))?;

        let parsed_hash = PasswordHash::new(encrypted_password).unwrap();
        info!(
            "Verify password: elapsed: {}ms",
            now.elapsed().as_millis().to_string()
        );
        Ok(argon2
            .verify_password(credential.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::{
        loremaster_configuration::{get_configuration_from_file, LoremasterConfiguration},
        password_encryption::{PasswordEncryption, PasswordEncryptionService},
    };
    use anyhow::Result;

    #[test]
    fn verify_same_key() -> Result<()> {
        let configuration: LoremasterConfiguration =
            get_configuration_from_file(&String::from("local"))?;
        let encryption_service = PasswordEncryptionService::new(
            configuration.hash_iterations,
            configuration.site_secret,
        );
        let encrypted_key: String = encryption_service.encrypt_password("input")?;
        let encrypted_key2: String = encryption_service.encrypt_password("input")?;
        let verify_result: bool = encryption_service.verify_password(&encrypted_key, "input")?;
        assert_ne!(encrypted_key, encrypted_key2);
        assert_ne!("input", encrypted_key);
        assert!(verify_result);
        Ok(())
    }

    #[test]
    fn verify_different_keys() -> Result<()> {
        let configuration: LoremasterConfiguration =
            get_configuration_from_file(&String::from("local"))?;
        let encryption_service = PasswordEncryptionService::new(
            configuration.hash_iterations,
            configuration.site_secret,
        );
        // The check function should return false if
        let verify_result: bool = encryption_service.verify_password(
            &encryption_service.encrypt_password("pancakes123!")?,
            "blueberries325&",
        )?;
        assert!(!verify_result);
        Ok(())
    }

    #[test]
    fn unique_encryption_check() -> Result<()> {
        let configuration: LoremasterConfiguration =
            get_configuration_from_file(&String::from("local"))?;
        let encryption_service = PasswordEncryptionService::new(
            configuration.hash_iterations,
            configuration.site_secret,
        );
        assert_ne!(
            encryption_service.encrypt_password("input")?,
            encryption_service.encrypt_password("input")?
        );
        Ok(())
    }
}
