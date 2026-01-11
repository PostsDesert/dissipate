use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    auth::{extract_token_from_header, validate_token},
    handlers::SharedState,
};

/// CORS layer configuration
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
            header::ACCEPT_ENCODING,
            header::ACCEPT_LANGUAGE,
            header::CACHE_CONTROL,
            header::PRAGMA,
            header::USER_AGENT,
        ])
        .allow_credentials(false)
}

/// Auth middleware - validates JWT and injects user_id into request extensions
pub async fn auth_middleware(
    State(state): State<SharedState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract and validate token
    let token = extract_token_from_header(auth_header).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = validate_token(token, &state.jwt_secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Insert user_id into request extensions
    request.extensions_mut().insert(claims.user_id);

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{auth::create_token, db, handlers::AppState};
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
        middleware::from_fn_with_state,
        response::IntoResponse,
        routing::get,
        Router,
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    async fn setup_test_state() -> SharedState {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        Arc::new(AppState {
            pool,
            jwt_secret: "test-secret".to_string(),
        })
    }

    async fn test_handler(request: Request<Body>) -> impl IntoResponse {
        let user_id = request
            .extensions()
            .get::<String>()
            .cloned()
            .unwrap_or_default();
        (StatusCode::OK, user_id)
    }

    fn create_test_router(state: SharedState) -> Router {
        Router::new()
            .route("/protected", get(test_handler))
            .layer(from_fn_with_state(state.clone(), auth_middleware))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_auth_middleware_valid_token() {
        let state = setup_test_state().await;
        let token = create_token("user-123", &state.jwt_secret).unwrap();

        let app = create_test_router(state);

        let request = Request::builder()
            .uri("/protected")
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_middleware_missing_header() {
        let state = setup_test_state().await;
        let app = create_test_router(state);

        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_invalid_token() {
        let state = setup_test_state().await;
        let app = create_test_router(state);

        let request = Request::builder()
            .uri("/protected")
            .header(header::AUTHORIZATION, "Bearer invalid-token")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_wrong_secret() {
        let state = setup_test_state().await;
        let token = create_token("user-123", "wrong-secret").unwrap();

        let app = create_test_router(state);

        let request = Request::builder()
            .uri("/protected")
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_basic_auth_rejected() {
        let state = setup_test_state().await;
        let app = create_test_router(state);

        let request = Request::builder()
            .uri("/protected")
            .header(header::AUTHORIZATION, "Basic dXNlcjpwYXNz")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_cors_layer_configuration() {
        let _cors = cors_layer();
        // Just verify it builds without error
        assert!(true);
    }

    #[tokio::test]
    async fn test_auth_middleware_injects_user_id() {
        let state = setup_test_state().await;
        let token = create_token("expected-user-id", &state.jwt_secret).unwrap();

        let app = create_test_router(state);

        let request = Request::builder()
            .uri("/protected")
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // The handler returns the user_id in the body
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let user_id = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(user_id, "expected-user-id");
    }
}
