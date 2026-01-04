use axum::{Json, http::StatusCode};
use serde_json::Value;

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(serde_json::json!({ "status": "ok" })))
}
