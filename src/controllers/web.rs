use crate::{handlers::User, utils::templates};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Form, Json, Router,
};
use serde::Deserialize;
use sqlx::{postgres::PgPool, Pool, Postgres};
use uuid::Uuid;

pub fn get_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/hello/:name", get(hello))
        .route("/laugh", get(laugh))
        .route("/users", get(users))
        .route("/users", post(add_users))
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

#[derive(Deserialize)]
struct NewUser {
    name: String,
}

async fn add_users(
    State(pool): State<PgPool>,
    Form(new_user): Form<NewUser>,
) -> (StatusCode, Html<String>) {
    let id = Uuid::new_v4();
    let name = new_user.name;
    let rows = sqlx::query_as::<_, User>(
        "insert into users (id, name) VALUES ($1, $2) RETURNING id, name",
    )
    .bind(id.to_string())
    .bind(name)
    .fetch_one(&pool)
    .await;
    let res: Result<Json<User>, sqlx::Error> = match rows {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            eprintln!("Error adding user: {:?}", err);
            Err(err)
        }
    };

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<div>Error</div>".to_string()),
        );
    }

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

    let temp = Html(templates::UsersListTemplate { users }.to_string());

    return (StatusCode::OK, temp);
}
