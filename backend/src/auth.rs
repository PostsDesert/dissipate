use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?;
    Ok(password_hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| anyhow!("Invalid password hash: {}", e))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}

pub fn generate_jwt(user_id: Uuid) -> Result<String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-key".to_string());
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(1296000)) // 15 days
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| anyhow!("Failed to generate token: {}", e))
}

pub fn validate_jwt(token: &str) -> Result<Claims> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-key".to_string());
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| anyhow!("Failed to validate token: {}", e))?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password_123";
        let hash = hash_password(password);
        assert!(hash.is_ok());
        assert!(!hash.unwrap().is_empty());
    }

    #[test]
    fn test_verify_password() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();

        let result = verify_password(&hash, password);
        assert!(result.is_ok());
        assert!(result.unwrap());

        let wrong_password = "wrong_password";
        let result = verify_password(&hash, wrong_password);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_generate_jwt() {
        let user_id = Uuid::new_v4();
        let token = generate_jwt(user_id);
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_validate_jwt() {
        let user_id = Uuid::new_v4();
        let token = generate_jwt(user_id).unwrap();

        let claims = validate_jwt(&token);
        assert!(claims.is_ok());
        assert_eq!(claims.unwrap().user_id, user_id);
    }

    #[test]
    fn test_validate_invalid_jwt() {
        let invalid_token = "invalid.token.here";
        let result = validate_jwt(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_expiration() {
        let user_id = Uuid::new_v4();
        let token = generate_jwt(user_id).unwrap();

        let claims = validate_jwt(&token).unwrap();
        let current_time = Utc::now().timestamp() as usize;
        assert!(claims.exp > current_time);
    }

    #[test]
    fn test_password_hash_uniqueness() {
        let password = "test_password_123";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        assert_ne!(hash1, hash2);
        assert!(verify_password(&hash1, password).unwrap());
        assert!(verify_password(&hash2, password).unwrap());
    }
}
