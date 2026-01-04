use axum::{extract::State, http::{header, StatusCode, Response}};
use sqlx::SqlitePool;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::db::get_user_messages;
use crate::auth::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMessage {
    pub id: Uuid,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn export_json(
    State(pool): State<SqlitePool>,
    _claims: Claims,
) -> Result<Response<String>, StatusCode> {
    match get_user_messages(&pool, &_claims.user_id).await {
        Ok(messages) => {
            let export_messages: Vec<ExportMessage> = messages
                .iter()
                .map(|(id, _user_id, content, created_at, updated_at)| ExportMessage {
                    id: *id,
                    content: content.clone(),
                    created_at: created_at.to_rfc3339(),
                    updated_at: updated_at.to_rfc3339(),
                })
                .collect();

            let json = serde_json::to_string(&export_messages)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::CONTENT_DISPOSITION, "attachment; filename=\"messages.json\"")
                .body(json)
                .unwrap())
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn export_markdown(
    State(pool): State<SqlitePool>,
    _claims: Claims,
) -> Result<Response<String>, StatusCode> {
    match get_user_messages(&pool, &_claims.user_id).await {
        Ok(messages) => {
            let mut markdown = String::from("# Messages Export\n\nExported: ");
            markdown.push_str(&chrono::Utc::now().format("%B %e, %Y").to_string());
            markdown.push_str("\n\n---\n\n");

            for (_, _user_id, content, created_at, _updated_at) in messages {
                markdown.push_str("## ");
                markdown.push_str(&created_at.format("%B %e, %Y at %I:%M %p").to_string());
                markdown.push_str("\n\n");
                markdown.push_str(content.as_str());
                markdown.push_str("\n\n---\n\n");
            }

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/markdown; charset=utf-8")
                .header(header::CONTENT_DISPOSITION, "attachment; filename=\"messages.md\"")
                .body(markdown)
                .unwrap())
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use crate::db;
    use crate::auth;

    async fn create_test_pool_and_user() -> (SqlitePool, Claims) {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        db::init_schema(&pool).await.unwrap();
        
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        db::create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let claims = Claims {
            user_id,
            exp: (chrono::Utc::now().timestamp() + 3600) as usize,
        };

        (pool, claims)
    }

    #[tokio::test]
    async fn test_export_message_serialization() {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        let export_msg = ExportMessage {
            id,
            content: "Test message".to_string(),
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
        };

        let json = serde_json::to_string(&export_msg);
        assert!(json.is_ok());
        
        let result = json.unwrap();
        assert!(result.contains("Test message"));
        assert!(result.contains(&id.to_string()));
    }

    #[tokio::test]
    async fn test_export_json_format() {
        let (pool, claims) = create_test_pool_and_user().await;
        
        let now = chrono::Utc::now();
        let message_id = Uuid::new_v4();
        db::create_message(
            &pool,
            &message_id,
            &claims.user_id,
            "Hello, world!",
            &now,
            &now,
        )
        .await
        .unwrap();

        match get_user_messages(&pool, &claims.user_id).await {
            Ok(messages) => {
                assert_eq!(messages.len(), 1);
                
                let export_messages: Vec<ExportMessage> = messages
                    .iter()
                    .map(|(id, _user_id, content, created_at, updated_at)| ExportMessage {
                        id: *id,
                        content: content.clone(),
                        created_at: created_at.to_rfc3339(),
                        updated_at: updated_at.to_rfc3339(),
                    })
                    .collect();

                assert_eq!(export_messages.len(), 1);
                assert_eq!(export_messages[0].content, "Hello, world!");
            }
            Err(_) => panic!("Failed to get messages"),
        }
    }

    #[tokio::test]
    async fn test_export_markdown_format() {
        let (pool, claims) = create_test_pool_and_user().await;
        
        let now = chrono::Utc::now();
        let message_id = Uuid::new_v4();
        db::create_message(
            &pool,
            &message_id,
            &claims.user_id,
            "Hello, world!",
            &now,
            &now,
        )
        .await
        .unwrap();

        match get_user_messages(&pool, &claims.user_id).await {
            Ok(messages) => {
                assert_eq!(messages.len(), 1);
                
                let mut markdown = String::from("# Messages Export\n\nExported: ");
                markdown.push_str(&chrono::Utc::now().format("%B %e, %Y").to_string());
                markdown.push_str("\n\n---\n\n");

                for (_, _user_id, content, created_at, _updated_at) in messages {
                    markdown.push_str("## ");
                    markdown.push_str(&created_at.format("%B %e, %Y at %I:%M %p").to_string());
                    markdown.push_str("\n\n");
                    markdown.push_str(content.as_str());
                    markdown.push_str("\n\n---\n\n");
                }

                assert!(markdown.contains("# Messages Export"));
                assert!(markdown.contains("Hello, world!"));
                assert!(markdown.contains("##"));
            }
            Err(_) => panic!("Failed to get messages"),
        }
    }

    #[tokio::test]
    async fn test_export_empty_messages() {
        let (pool, claims) = create_test_pool_and_user().await;
        
        match get_user_messages(&pool, &claims.user_id).await {
            Ok(messages) => {
                assert_eq!(messages.len(), 0);
            }
            Err(_) => panic!("Failed to get messages"),
        }
    }
}
