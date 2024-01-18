use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::{Error, SaltString};
use rand_core::OsRng;

pub trait SecureHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, SecureHasherError>;
    fn verify_password(&self, password: &str, saved_hash: &str) -> Result<(), SecureHasherError>;
}

#[derive(Debug)]
pub enum SecureHasherError {
    UnexpectedError(anyhow::Error),

    WrongPassword,
}

pub struct Argon2Hasher;

impl SecureHasher for Argon2Hasher {
    fn hash_password(&self, password: &str) -> Result<String, SecureHasherError> {
        let password_salt = SaltString::generate(&mut OsRng);

        let argon2_params = Params::new(8192, 5, 1, Some(32))
            .map_err(|e| SecureHasherError::UnexpectedError(e.into()))?;

        Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params)
            .hash_password(password.as_ref(), &password_salt)
            .map(|hash| hash.to_string())
            .map_err(|e| SecureHasherError::UnexpectedError(e.into()))
    }

    fn verify_password(&self, password: &str, saved_hash: &str) -> Result<(), SecureHasherError> {
        let parsed_hash = PasswordHash::new(saved_hash)
            .map_err(|e| SecureHasherError::UnexpectedError(e.into()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                Error::Password => SecureHasherError::WrongPassword,
                _ => SecureHasherError::UnexpectedError(e.into())
            })
    }
}