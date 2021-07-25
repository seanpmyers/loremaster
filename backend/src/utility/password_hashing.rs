use anyhow::{anyhow, Result};
use base64;
use rand::SystemRandom;
use ring::rand::SecureRandom;
use ring::{pbkdf2, rand};
use std::num::NonZeroU32;
//TODO: Fix trait 
pub trait IHashEncryptionService {
    const SALT_SIZE: usize;
    const KEY_SIZE: usize;
    const ITERATIONS: u32;
    fn new() -> Self;
    fn hash(input: &str) -> Result<String>;
    fn check(hash: &str, input: &str) -> Result<(bool, bool)>;
}

pub struct HashEncryptionService();

impl IHashEncryptionService for HashEncryptionService {
    const SALT_SIZE: usize = 16;
    const KEY_SIZE: usize = 32;
    const ITERATIONS: u32 = 10000;

    fn new() -> HashEncryptionService {
        return HashEncryptionService {};
    }

    fn hash(input: &str) -> Result<String> {
        let iterations = NonZeroU32::new(Self::ITERATIONS).unwrap();
        let random_number_generator = SystemRandom::new();

        let mut salt = [0u8; Self::SALT_SIZE];

        random_number_generator.fill(&mut salt).unwrap();

        let mut pbkdf2_hash = [0u8; Self::KEY_SIZE];

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            &salt,
            input.as_bytes(),
            &mut pbkdf2_hash,
        );

        //format: {iterations.salt.key}
        let hashed_input = format!(
            "{}.{}.{}",
            Self::ITERATIONS,
            base64::encode(&salt),
            base64::encode(&pbkdf2_hash)
        );

        return Ok(hashed_input);
    }

    fn check(hash: &str, input: &str) -> Result<(bool, bool)> {
        let hash_parts: Vec<&str> = hash.split(".").collect();
        if hash_parts.len() < 3 {
            return Err(anyhow!("Unable to split provided hash string!"));
        }

        let iterations = NonZeroU32::new(hash_parts[0].parse::<u32>()?).unwrap();
        let salt: Vec<u8> = base64::decode(hash_parts[1])?;
        let key: Vec<u8> = base64::decode(hash_parts[2])?;

        let needs_upgrade: bool = false;

        let mut pbkdf2_hash = [0u8; Self::KEY_SIZE];

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            &salt,
            input.as_bytes(),
            &mut pbkdf2_hash,
        );

        let is_equivalent: bool = key
            .iter()
            .zip(pbkdf2_hash.iter())
            .all(|(input, computed)| input == computed);

        return Ok((is_equivalent, needs_upgrade));
    }
}

#[cfg(test)]
mod tests {

    use super::{HashEncryptionService, IHashEncryptionService};

    #[test]
    fn test1() {
        let hashed_key: String = HashEncryptionService::hash("input").unwrap();
        let hashed_key2: String = HashEncryptionService::hash("input").unwrap();
        let check = HashEncryptionService::check(&hashed_key, "input").unwrap();
        println!("{}", check.1);
        assert_ne!(hashed_key, hashed_key2);
        assert_ne!("input", hashed_key);
        assert_eq!(check.0, true);
    }

    #[test]
    fn check_different_keys() {
        // The check function should return false if
        let hashed_key: String = HashEncryptionService::hash("pancakes123!").unwrap();
        let check_result: (bool, bool) =
            HashEncryptionService::check(&hashed_key, "blueberries325&").unwrap();
        assert_eq!(check_result.0, false);
    }

    #[test]
    fn hash_same_input() {}
}
