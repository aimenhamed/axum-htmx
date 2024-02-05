use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use uuid::Uuid;

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

pub async fn add_user(State(pool): State<PgPool>, Json(user): Json<CreateUser>) -> Json<User> {
    let id = Uuid::new_v4();
    let rows = sqlx::query_as::<_, User>(
        "insert into users (id, name) VALUES ($1, $2) RETURNING id, name",
    )
    .bind(id.to_string())
    .bind(user.name)
    .fetch_one(&pool)
    .await;
    let res: Result<Json<User>, sqlx::Error> = match rows {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            eprintln!("Error adding user: {:?}", err);
            Err(err)
        }
    };

    return res.unwrap();
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
}
