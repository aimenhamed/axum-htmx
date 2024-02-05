use crate::{handlers::User, utils::templates};
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Json, Router,
};
use sqlx::{postgres::PgPool, Pool, Postgres};

pub fn get_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/hello/:name", get(hello))
        .route("/laugh", get(laugh))
        .route("/users", get(users))
        .with_state(pool)
}

async fn hello(Path(name): Path<String>) -> Html<String> {
    Html(templates::HelloTemplate { name }.to_string())
}

async fn laugh() -> Html<String> {
    Html("<p>HAHAHA</p>".into())
}

async fn users(State(pool): State<PgPool>) -> Html<String> {
    let rows = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await;

    let res: Result<Json<Vec<User>>, sqlx::Error> = match rows {
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            Err(err)
        }
    };
    let users = res.unwrap().to_vec();

    Html(templates::UsersTemplate { users }.to_string())
}
