use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, PasswordHash, rand_core::OsRng}};
use crate::helpers::errors::AppError;


pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::HashingFailed(e.to_string()))
}


pub fn verify_password(hash: &str, password: &str) -> Result<(),AppError> { 
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::HashingFailed(e.to_string()))?;
    let argon2 = Argon2::default();

    argon2.verify_password(password.as_bytes(), &parsed_hash).map_err(|_| AppError::VerificationFailed)
}