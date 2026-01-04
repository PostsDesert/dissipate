use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    pub id: Option<Uuid>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsernameRequest {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct MessagesQuery {
    pub since: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessagesResponse {
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_user_creation() {
        let id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let username = "testuser".to_string();
        let now = Utc::now();

        let user = User {
            id,
            email: email.clone(),
            username: username.clone(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(user.id, id);
        assert_eq!(user.email, email);
        assert_eq!(user.username, username);
    }

    #[test]
    fn test_message_creation() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let content = "Hello, world!".to_string();
        let now = Utc::now();

        let message = Message {
            id,
            user_id,
            content: content.clone(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(message.id, id);
        assert_eq!(message.user_id, user_id);
        assert_eq!(message.content, content);
    }

    #[test]
    fn test_login_request() {
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "password123");
    }

    #[test]
    fn test_create_message_request() {
        let request = CreateMessageRequest {
            id: Some(Uuid::new_v4()),
            content: "Hello, world!".to_string(),
        };

        assert!(!request.content.is_empty());
        assert!(request.id.is_some());
    }

    #[test]
    fn test_update_message_request() {
        let request = UpdateMessageRequest {
            content: "Updated content".to_string(),
        };

        assert_eq!(request.content, "Updated content");
    }

    #[test]
    fn test_update_email_request() {
        let request = UpdateEmailRequest {
            email: "newemail@example.com".to_string(),
        };

        assert_eq!(request.email, "newemail@example.com");
    }

    #[test]
    fn test_update_username_request() {
        let request = UpdateUsernameRequest {
            username: "newusername".to_string(),
        };

        assert_eq!(request.username, "newusername");
    }

    #[test]
    fn test_update_password_request() {
        let request = UpdatePasswordRequest {
            current_password: "oldpassword".to_string(),
            new_password: "newpassword".to_string(),
        };

        assert_eq!(request.current_password, "oldpassword");
        assert_eq!(request.new_password, "newpassword");
    }

    #[test]
    fn test_claims_serialization() {
        let user_id = Uuid::new_v4();
        let claims = Claims {
            user_id,
            exp: 1735689600,
        };

        let user_id_claims = claims.user_id;
        assert_eq!(user_id_claims, user_id);
        assert_eq!(claims.exp, 1735689600);
    }
}
