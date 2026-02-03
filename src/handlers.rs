use axum::Json;

use crate::models::HealthResponse;

// Handler that respond with static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

// Handler for health check
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "Server healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}