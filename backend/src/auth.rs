use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use thiserror::Error;

use crate::models::Claims;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to create token: {0}")]
    TokenCreationError(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Token expired")]
    TokenExpired,
    #[allow(dead_code)]
    #[error("Missing authorization header")]
    MissingAuthHeader,
    #[error("Invalid authorization header format")]
    InvalidAuthHeader,
}

/// Token expiration in days
const TOKEN_EXPIRATION_DAYS: i64 = 15;

/// Create a JWT token for the given user ID
pub fn create_token(user_id: &str, secret: &str) -> Result<String, AuthError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(TOKEN_EXPIRATION_DAYS))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::TokenCreationError(e.to_string()))
}

/// Validate a JWT token and return the claims
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        if e.to_string().contains("ExpiredSignature") {
            AuthError::TokenExpired
        } else {
            AuthError::InvalidToken(e.to_string())
        }
    })?;

    Ok(token_data.claims)
}

/// Extract token from Authorization header
pub fn extract_token_from_header(auth_header: &str) -> Result<&str, AuthError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::InvalidAuthHeader);
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();
    if token.is_empty() {
        return Err(AuthError::InvalidAuthHeader);
    }

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test-secret-key-for-testing";

    #[test]
    fn test_create_token_returns_valid_jwt() {
        let user_id = "user-123";

        let token = create_token(user_id, TEST_SECRET).unwrap();

        // JWT tokens have 3 parts separated by dots
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        // Header and payload parts should be non-empty
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
        assert!(!parts[2].is_empty());
    }

    #[test]
    fn test_validate_token_returns_correct_claims() {
        let user_id = "user-456";

        let token = create_token(user_id, TEST_SECRET).unwrap();
        let claims = validate_token(&token, TEST_SECRET).unwrap();

        assert_eq!(claims.user_id, user_id);
        assert!(claims.exp > Utc::now().timestamp() as usize);
    }

    #[test]
    fn test_validate_token_fails_with_wrong_secret() {
        let user_id = "user-789";

        let token = create_token(user_id, TEST_SECRET).unwrap();
        let result = validate_token(&token, "wrong-secret");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::InvalidToken(_)));
    }

    #[test]
    fn test_validate_token_fails_with_invalid_token() {
        let result = validate_token("invalid.token.here", TEST_SECRET);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_fails_with_malformed_token() {
        let result = validate_token("not-a-jwt", TEST_SECRET);

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_token_from_header_valid() {
        let header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test.signature";

        let token = extract_token_from_header(header).unwrap();

        assert_eq!(token, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test.signature");
    }

    #[test]
    fn test_extract_token_from_header_fails_without_bearer() {
        let header = "Basic dXNlcjpwYXNz";

        let result = extract_token_from_header(header);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::InvalidAuthHeader));
    }

    #[test]
    fn test_extract_token_from_header_fails_with_empty_token() {
        let header = "Bearer ";

        let result = extract_token_from_header(header);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::InvalidAuthHeader));
    }

    #[test]
    fn test_extract_token_from_header_trims_whitespace() {
        let header = "Bearer   token-with-spaces   ";

        let token = extract_token_from_header(header).unwrap();

        assert_eq!(token, "token-with-spaces");
    }

    #[test]
    fn test_create_token_different_users_get_different_tokens() {
        let token1 = create_token("user-1", TEST_SECRET).unwrap();
        let token2 = create_token("user-2", TEST_SECRET).unwrap();

        assert_ne!(token1, token2);
    }

    #[test]
    fn test_token_expiration_is_in_future() {
        let token = create_token("user-123", TEST_SECRET).unwrap();
        let claims = validate_token(&token, TEST_SECRET).unwrap();

        let now = Utc::now().timestamp() as usize;
        let expected_min_exp = now + (14 * 24 * 60 * 60); // At least 14 days in future

        assert!(claims.exp > expected_min_exp);
    }
}
