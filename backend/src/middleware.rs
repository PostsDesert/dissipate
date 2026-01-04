use axum::extract::Request;
use tower_http::cors::CorsLayer;

pub fn create_cors() -> CorsLayer {
    CorsLayer::new()
}
