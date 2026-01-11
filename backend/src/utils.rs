use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Failed to hash password: {0}")]
    HashError(String),
    #[error("Failed to verify password: {0}")]
    VerifyError(String),
}

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<(String, String), PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| PasswordError::HashError(e.to_string()))?
        .to_string();

    Ok((password_hash, salt.to_string()))
}

/// Verify a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| PasswordError::VerifyError(e.to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_creates_unique_hashes() {
        let password = "test_password_123";

        let (hash1, salt1) = hash_password(password).unwrap();
        let (hash2, salt2) = hash_password(password).unwrap();

        // Each hash should be unique due to random salt
        assert_ne!(hash1, hash2);
        assert_ne!(salt1, salt2);

        // Hashes should not be empty
        assert!(!hash1.is_empty());
        assert!(!salt1.is_empty());
    }

    #[test]
    fn test_hash_password_produces_argon2_format() {
        let password = "my_secure_password";

        let (hash, _salt) = hash_password(password).unwrap();

        // Argon2 hashes start with $argon2
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_password_succeeds_with_correct_password() {
        let password = "correct_password";

        let (hash, _salt) = hash_password(password).unwrap();
        let result = verify_password(password, &hash).unwrap();

        assert!(result);
    }

    #[test]
    fn test_verify_password_fails_with_wrong_password() {
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let (hash, _salt) = hash_password(password).unwrap();
        let result = verify_password(wrong_password, &hash).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_verify_password_handles_empty_password() {
        let password = "";

        let (hash, _salt) = hash_password(password).unwrap();
        let result = verify_password(password, &hash).unwrap();

        assert!(result);
    }

    #[test]
    fn test_verify_password_handles_special_characters() {
        let password = "p@$$w0rd!#$%^&*(){}[]|\\:\";<>,.?/~`";

        let (hash, _salt) = hash_password(password).unwrap();
        let result = verify_password(password, &hash).unwrap();

        assert!(result);
    }

    #[test]
    fn test_verify_password_handles_unicode() {
        let password = "密码🔐パスワード";

        let (hash, _salt) = hash_password(password).unwrap();
        let result = verify_password(password, &hash).unwrap();

        assert!(result);
    }

    #[test]
    fn test_verify_password_fails_with_invalid_hash() {
        let result = verify_password("password", "invalid_hash");

        assert!(result.is_err());
    }

    #[test]
    fn test_hash_password_handles_long_passwords() {
        // 1000 character password
        let password = "a".repeat(1000);

        let (hash, _salt) = hash_password(&password).unwrap();
        let result = verify_password(&password, &hash).unwrap();

        assert!(result);
    }
}
