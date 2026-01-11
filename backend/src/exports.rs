use axum::{
    extract::State,
    http::{header, StatusCode},
    response::Response,
    Json,
};
use chrono::{DateTime, Utc};

use crate::{
    db,
    handlers::{ErrorResponse, SharedState},
    models::MessageResponse,
};

/// GET /api/export/json
/// Export all user messages as JSON
pub async fn export_json(
    State(state): State<SharedState>,
    user_id: String,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let messages = db::get_messages_for_user(&state.pool, &user_id, None)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("Failed to fetch messages"),
            )
        })?;

    let message_responses: Vec<MessageResponse> =
        messages.iter().map(|m| m.to_response()).collect();

    let json = serde_json::to_string_pretty(&message_responses).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new("Failed to serialize messages"),
        )
    })?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"messages.json\"",
        )
        .body(json.into())
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("Failed to build response"),
            )
        })?;

    Ok(response)
}

/// GET /api/export/markdown
/// Export all user messages as Markdown
pub async fn export_markdown(
    State(state): State<SharedState>,
    user_id: String,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let messages = db::get_messages_for_user(&state.pool, &user_id, None)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("Failed to fetch messages"),
            )
        })?;

    let now = Utc::now();
    let export_date = now.format("%B %d, %Y").to_string();

    let mut markdown = format!("# Messages Export\n\nExported: {}\n\n---\n\n", export_date);

    for message in messages {
        // Parse the created_at timestamp
        let formatted_date = if let Ok(dt) = DateTime::parse_from_rfc3339(&message.created_at) {
            dt.format("%B %d, %Y at %I:%M %p").to_string()
        } else {
            message.created_at.clone()
        };

        markdown.push_str(&format!(
            "## {}\n\n{}\n\n---\n\n",
            formatted_date, message.content
        ));
    }

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/markdown; charset=utf-8")
        .header(
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"messages.md\"",
        )
        .body(markdown.into())
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("Failed to build response"),
            )
        })?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db,
        handlers::AppState,
        models::Message,
        utils::hash_password,
    };
    use http_body_util::BodyExt;
    use std::sync::Arc;

    async fn setup_test_state() -> SharedState {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        Arc::new(AppState {
            pool,
            jwt_secret: "test-secret".to_string(),
        })
    }

    async fn create_test_user(state: &SharedState, email: &str) -> crate::models::User {
        let (hash, salt) = hash_password("password123").unwrap();
        let user = crate::models::User::new(email.to_string(), "testuser".to_string(), hash, salt);
        db::create_user(&state.pool, &user).await.unwrap();
        user
    }

    #[tokio::test]
    async fn test_export_json_empty() {
        let state = setup_test_state().await;
        let user = create_test_user(&state, "export@example.com").await;

        let result = export_json(State(state), user.id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Check content-type header
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap();
        assert_eq!(content_type, "application/json");

        // Check content-disposition header
        let content_disposition = response.headers().get(header::CONTENT_DISPOSITION).unwrap();
        assert!(content_disposition
            .to_str()
            .unwrap()
            .contains("messages.json"));
    }

    #[tokio::test]
    async fn test_export_json_with_messages() {
        let state = setup_test_state().await;
        let user = create_test_user(&state, "jsonexport@example.com").await;

        // Create some messages
        let msg1 = Message::new(user.id.clone(), "First message".to_string());
        let msg2 = Message::new(user.id.clone(), "Second message".to_string());
        db::create_message(&state.pool, &msg1).await.unwrap();
        db::create_message(&state.pool, &msg2).await.unwrap();

        let result = export_json(State(state), user.id).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        // Parse body
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let json_str = String::from_utf8(bytes.to_vec()).unwrap();

        let messages: Vec<MessageResponse> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn test_export_markdown_empty() {
        let state = setup_test_state().await;
        let user = create_test_user(&state, "mdexport@example.com").await;

        let result = export_markdown(State(state), user.id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Check content-type header
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap();
        assert!(content_type.to_str().unwrap().contains("text/markdown"));

        // Check content-disposition header
        let content_disposition = response.headers().get(header::CONTENT_DISPOSITION).unwrap();
        assert!(content_disposition
            .to_str()
            .unwrap()
            .contains("messages.md"));
    }

    #[tokio::test]
    async fn test_export_markdown_with_messages() {
        let state = setup_test_state().await;
        let user = create_test_user(&state, "mdwithmsg@example.com").await;

        let msg = Message::new(user.id.clone(), "Test message content".to_string());
        db::create_message(&state.pool, &msg).await.unwrap();

        let result = export_markdown(State(state), user.id).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let markdown = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(markdown.contains("# Messages Export"));
        assert!(markdown.contains("Test message content"));
    }

    #[tokio::test]
    async fn test_export_markdown_format() {
        let state = setup_test_state().await;
        let user = create_test_user(&state, "mdformat@example.com").await;

        let msg = Message::new(user.id.clone(), "My test message".to_string());
        db::create_message(&state.pool, &msg).await.unwrap();

        let result = export_markdown(State(state), user.id).await;

        let response = result.unwrap();
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let markdown = String::from_utf8(bytes.to_vec()).unwrap();

        // Check structure
        assert!(markdown.starts_with("# Messages Export"));
        assert!(markdown.contains("Exported:"));
        assert!(markdown.contains("---"));
        assert!(markdown.contains("##")); // Date headers
        assert!(markdown.contains("My test message"));
    }
}
