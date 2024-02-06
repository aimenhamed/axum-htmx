use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn health_check() -> (StatusCode, &'static str) {
    tracing::info!("health check");
    (StatusCode::OK, "healthy")
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
}
