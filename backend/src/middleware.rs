use tower_http::cors::CorsLayer;
use axum::{
    http::request::Parts,
    response::Response,
};
use async_trait::async_trait;
use axum::extract::FromRequestParts;

use crate::auth::Claims;

pub fn create_cors() -> CorsLayer {
    CorsLayer::new()
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        
        let auth_header = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                let mut res = Response::new("Missing Authorization header".into());
                *res.status_mut() = axum::http::StatusCode::UNAUTHORIZED;
                res
            })?;

        if !auth_header.starts_with("Bearer ") {
            let mut res = Response::new("Invalid Authorization header format".into());
            *res.status_mut() = axum::http::StatusCode::UNAUTHORIZED;
            return Err(res);
        }

        let token = &auth_header[7..];
        
        match crate::auth::validate_jwt(token) {
            Ok(claims) => Ok(claims),
            Err(_) => {
                let mut res = Response::new("Invalid token".into());
                *res.status_mut() = axum::http::StatusCode::UNAUTHORIZED;
                Err(res)
            }
        }
    }
}
