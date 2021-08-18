use anyhow::{anyhow, Result};
use base64;
use rand::SystemRandom;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use crate::utility::toml_reader::{self, SECRET_FILE_PATH};

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const ITERATIONS_FIELD : &str = "HASH_ITERATIONS";
const SITE_SECRET_FIELD : &str = "SITE_SECRET";
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
pub type Credential = [u8; CREDENTIAL_LEN];

//TODO: redo this using ring documentation example 
pub struct HashEncryptionService {
    salt_size: usize,
    key_size: usize,
    iterations: u32,
}

impl HashEncryptionService {
    const SALT_SIZE: usize = 16;
    const KEY_SIZE: usize = 32;

    pub fn hash(input: &str) -> Result<String> {
        let site_secret: String = toml_reader::get_toml_field_value(SECRET_FILE_PATH, SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = toml_reader::get_toml_field_value(SECRET_FILE_PATH, ITERATIONS_FIELD)?.parse::<u32>()?;

        let iterations = NonZeroU32::new(iterations_setting).unwrap();
        let random_number_generator = SystemRandom::new();

        let mut salt = [0u8; Self::SALT_SIZE];
        random_number_generator.fill(&mut salt).unwrap();

        let mut hashed_secret = [0u8; Self::SALT_SIZE];
        pbkdf2::derive(
            PBKDF2_ALG, 
            iterations, 
            &salt, 
            site_secret.as_bytes(), 
            &mut hashed_secret
        );

        let mut hashed_input = [0u8; Self::KEY_SIZE];
        pbkdf2::derive(
            PBKDF2_ALG,
            iterations,
            &hashed_secret,
            input.as_bytes(),
            &mut hashed_input,
        );

        //format: {iterations.salt.key}
        let hashed_input = format!(
            "{}.{}.{}",
            iterations_setting,
            base64::encode(&salt),
            base64::encode(&hashed_input)
        );

        return Ok(hashed_input);
    }

    pub fn verify(hash: &str, user_input: &str) -> Result<(bool, bool)> {
        let site_secret: String = toml_reader::get_toml_field_value(SECRET_FILE_PATH, SITE_SECRET_FIELD)?;
        let iterations_setting: u32 = toml_reader::get_toml_field_value(SECRET_FILE_PATH, ITERATIONS_FIELD)?.parse::<u32>()?;

        let hash_parts: Vec<&str> = hash.split(".").collect();
        if hash_parts.len() < 3 {
            return Err(anyhow!("Unable to split provided hash string!"));
        }

        let stored_iterations: u32 = hash_parts[0].parse::<u32>()?;

        let iterations = NonZeroU32::new(stored_iterations).unwrap();
        let salt: Vec<u8> = base64::decode(hash_parts[1])?;
        let key: Vec<u8> = base64::decode(hash_parts[2])?;

        let mut hashed_secret = [0u8; Self::SALT_SIZE];
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

    use super::{HashEncryptionService};

    #[test]
    fn verify_same_key() {
        let hashed_key: String = HashEncryptionService::hash("input").unwrap();
        let hashed_key2: String = HashEncryptionService::hash("input").unwrap();
        let check = HashEncryptionService::verify(&hashed_key, "input").unwrap();
        // println!("{}", check.1);
        assert_ne!(hashed_key, hashed_key2);
        assert_ne!("input", hashed_key);
        assert_eq!(check.0, true);
    }

    #[test]
    fn verify_different_keys() {
        // The check function should return false if
        let hashed_key: String = HashEncryptionService::hash("pancakes123!").unwrap();
        let check_result: (bool, bool) = HashEncryptionService::verify(&hashed_key, "blueberries325&").unwrap();
        assert_eq!(check_result.0, false);
    }

    #[test]
    fn hash_same_input() {

    }
}
