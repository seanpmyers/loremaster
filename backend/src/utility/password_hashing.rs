use anyhow::{anyhow, Result};
use base64;
use rand::SystemRandom;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use crate::utility::secret_service::{ITERATIONS_FIELD, SITE_SECRET_FIELD, get_secret};

const SALT_SIZE: usize = 16;
const CREDENTIAL_LENGTH: usize = digest::SHA256_OUTPUT_LEN;

pub type Credential = [u8; CREDENTIAL_LENGTH];
pub type Salt = [u8; SALT_SIZE];

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
pub struct PasswordEncryptionService {}

pub trait PasswordBasedHashEncryption {
    fn hash_password(input: &str) -> Result<String>;
    fn verify_password(hash: &str, user_input: &str) -> Result<(bool, bool)>;
}

impl PasswordEncryptionService {
    //format: {iterations.salt.key}
    pub fn format_hashed_password(iterations_count: &u32, salt: &Salt, hashed_password: &Credential) -> String {
        return format!(
            "{}.{}.{}",
            iterations_count,
            base64::encode(&salt),
            base64::encode(&hashed_password)
        );
    }
}

impl PasswordBasedHashEncryption for PasswordEncryptionService {

    fn hash_password(credential: &str) -> Result<String> {
        let site_secret: String = get_secret( SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = get_secret( ITERATIONS_FIELD)?.parse::<u32>()?;

        let iterations = NonZeroU32::new(iterations_setting).unwrap();
        let random_number_generator = SystemRandom::new();

        let mut salt: Salt = [0u8; SALT_SIZE];
        random_number_generator.fill(&mut salt).unwrap(); // How should unspecified errors be handeled

        //To improve security of the salt, we hash our site secret and use it as the password's salt
        let mut hashed_secret: Salt = [0u8; SALT_SIZE];
        pbkdf2::derive(
            PBKDF2_ALG, 
            iterations, 
            &salt, 
            site_secret.as_bytes(), 
            &mut hashed_secret
        );

        let mut hashed_credential: Credential = [0u8; CREDENTIAL_LENGTH];
        pbkdf2::derive(
            PBKDF2_ALG,
            iterations,
            &hashed_secret,
            credential.as_bytes(),
            &mut hashed_credential,
        );

        let hashed_input = Self::format_hashed_password(&iterations_setting, &salt, &hashed_credential);

        return Ok(hashed_input);
    }

    fn verify_password(hash: &str, user_input: &str) -> Result<(bool, bool)> {
        let site_secret: String = get_secret(SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = get_secret(ITERATIONS_FIELD)?.parse::<u32>()?;

        let hash_parts: Vec<&str> = hash.split(".").collect();
        if hash_parts.len() < 3 {
            return Err(anyhow!("Unable to split provided hash string!"));
        }

        let stored_iterations: u32 = hash_parts[0].parse::<u32>()?;

        let iterations = NonZeroU32::new(stored_iterations).unwrap();
        let salt: Vec<u8> = base64::decode(hash_parts[1])?;
        let key: Vec<u8> = base64::decode(hash_parts[2])?;

        let mut hashed_secret = [0u8; SALT_SIZE];
        pbkdf2::derive(
            PBKDF2_ALG, 
            iterations, 
            &salt, 
            site_secret.as_bytes(), 
            &mut hashed_secret
        );

        let needs_upgrade: bool = stored_iterations != iterations_setting;

        let verify_result = pbkdf2::verify(
            PBKDF2_ALG,
            iterations,
            &hashed_secret,
            user_input.as_bytes(),
            &key,
        );

        match verify_result {
            Ok(_) => return Ok((true, needs_upgrade)),
            Err(_) => return Ok((false, needs_upgrade)),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::utility::password_hashing::{PasswordBasedHashEncryption, PasswordEncryptionService};

    #[test]
    fn verify_same_key() {
        let hashed_key: String = PasswordEncryptionService::hash_password("input").unwrap();
        let hashed_key2: String = PasswordEncryptionService::hash_password("input").unwrap();
        let check = PasswordEncryptionService::verify_password(&hashed_key, "input").unwrap();
        // println!("{}", check.1);
        assert_ne!(hashed_key, hashed_key2);
        assert_ne!("input", hashed_key);
        assert_eq!(check.0, true);
    }

    #[test]
    fn verify_different_keys() {
        // The check function should return false if
        let hashed_key: String = PasswordEncryptionService::hash_password("pancakes123!").unwrap();
        let check_result: (bool, bool) = PasswordEncryptionService::verify_password(&hashed_key, "blueberries325&").unwrap();
        assert_eq!(check_result.0, false);
    }

    #[test]
    fn hash_same_input() {

    }
}
