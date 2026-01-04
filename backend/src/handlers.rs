use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
    routing::{get, post, put, delete},
    extract::{Path, State, Query},
};
use sqlx::SqlitePool;
use uuid::Uuid;
use serde_json::json;

use crate::{
    models::{
        User, Message, LoginRequest, LoginResponse, CreateMessageRequest,
        UpdateMessageRequest, SuccessResponse, UpdateEmailRequest,
        UpdateUsernameRequest, UpdatePasswordRequest, MessagesQuery,
        MessagesResponse,
    },
    auth::{hash_password, verify_password, generate_jwt, Claims},
    db::{
        create_pool, init_schema, get_user_by_email, get_user_by_id,
        update_user_email, update_user_username, update_user_password,
        create_message, get_user_messages, get_message,
        update_message, delete_message,
    },
    exports::{export_json, export_markdown},
};

pub type AppState = SqlitePool;

pub async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn login(
    State(pool): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, StatusCode> {
    match get_user_by_email(&pool, &req.email).await {
        Ok(Some(user)) => {
            let (id, email, username, password_hash, _salt, created_at, updated_at) = user;
            
            match verify_password(&password_hash, &req.password) {
                Ok(true) => {
                    match generate_jwt(id) {
                        Ok(token) => {
                            let user_response = User {
                                id,
                                email,
                                username,
                                created_at,
                                updated_at,
                            };
                            
                            let response = LoginResponse {
                                token,
                                user: user_response,
                            };
                            
                            Ok((StatusCode::OK, Json(response)).into_response())
                        }
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                Ok(false) => Err(StatusCode::UNAUTHORIZED),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_messages(
    State(pool): State<AppState>,
    _claims: Claims,
    Query(_query): Query<MessagesQuery>,
) -> Result<Json<MessagesResponse>, StatusCode> {
    match get_user_messages(&pool, &_claims.user_id).await {
        Ok(messages) => {
            let messages_vec = messages
                .into_iter()
                .map(|(id, user_id, content, created_at, updated_at)| Message {
                    id,
                    user_id,
                    content,
                    created_at,
                    updated_at,
                })
                .collect();
            
            Ok(Json(MessagesResponse {
                messages: messages_vec,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_message_handler(
    State(pool): State<AppState>,
    claims: Claims,
    Json(req): Json<CreateMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    let now = chrono::Utc::now();
    let message_id = req.id.unwrap_or_else(Uuid::new_v4);
    
    match create_message(
        &pool,
        &message_id,
        &claims.user_id,
        &req.content,
        &now,
        &now,
    )
    .await
    {
        Ok(_) => {
            let message = Message {
                id: message_id,
                user_id: claims.user_id,
                content: req.content,
                created_at: now,
                updated_at: now,
            };
            Ok(Json(message))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_message_handler(
    State(pool): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMessageRequest>,
) -> Result<Response, StatusCode> {
    match get_message(&pool, &id).await {
        Ok(Some(message)) => {
            let (_msg_id, user_id, _content, created_at, _updated_at) = message;
            
            if user_id != claims.user_id {
                return Err(StatusCode::FORBIDDEN);
            }
            
            match update_message(&pool, &id, &req.content).await {
                Ok(_) => {
                    let now = chrono::Utc::now();
                    let message = Message {
                        id,
                        user_id: claims.user_id,
                        content: req.content,
                        created_at,
                        updated_at: now,
                    };
                    Ok((StatusCode::OK, Json(message)).into_response())
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_message_handler(
    State(pool): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    match get_message(&pool, &id).await {
        Ok(Some(message)) => {
            let (_msg_id, user_id, _, _, _) = message;
            
            if user_id != claims.user_id {
                return Err(StatusCode::FORBIDDEN);
            }
            
            match delete_message(&pool, &id).await {
                Ok(_) => Ok(Json(SuccessResponse { success: true })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_email_handler(
    State(pool): State<AppState>,
    claims: Claims,
    Json(req): Json<UpdateEmailRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    match update_user_email(&pool, &claims.user_id, &req.email).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_username_handler(
    State(pool): State<AppState>,
    claims: Claims,
    Json(req): Json<UpdateUsernameRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    match update_user_username(&pool, &claims.user_id, &req.username).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_password_handler(
    State(pool): State<AppState>,
    claims: Claims,
    Json(req): Json<UpdatePasswordRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    match get_user_by_id(&pool, &claims.user_id).await {
        Ok(Some(_)) => {
            let salt = uuid::Uuid::new_v4().to_string();
            let password_hash = hash_password(&req.new_password).unwrap();
            
            match update_user_password(&pool, &claims.user_id, &password_hash, &salt).await {
                Ok(_) => Ok(Json(SuccessResponse { success: true })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn run_server() {
    let pool = create_pool().await.expect("Failed to create database pool");
    init_schema(&pool).await.expect("Failed to initialize database schema");
    
    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind address");
    println!("Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

fn create_app(pool: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/login", post(login))
        .route("/api/messages", get(get_messages).post(create_message_handler))
        .route("/api/messages/:id", put(update_message_handler).delete(delete_message_handler))
        .route("/api/user/email", put(update_email_handler))
        .route("/api/user/username", put(update_username_handler))
        .route("/api/user/password", put(update_password_handler))
        .route("/api/export/json", get(export_json))
        .route("/api/export/markdown", get(export_markdown))
        .layer(crate::middleware::create_cors())
        .with_state(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Method, header};
    use tower::ServiceExt;
    use crate::db;
    use crate::auth;

    async fn create_test_app() -> Router {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        db::init_schema(&pool).await.unwrap();
        create_app(pool)
    }

    async fn create_test_user(pool: &SqlitePool) -> Claims {
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        db::create_user(
            pool,
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

        Claims {
            user_id,
            exp: (chrono::Utc::now().timestamp() + 3600) as usize,
        }
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = create_test_app().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_success() {
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

        let app = create_app(pool);
        
        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/login")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
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

        let app = create_app(pool);
        
        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/login")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
