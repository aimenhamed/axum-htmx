use axum::{
    routing::{get, post},
    Router,
};
use handlers::{add_user, handler_404, health_check, root};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod controllers;
mod handlers;
mod utils;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/mydb".to_string());

    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let cloned_pool = pool.clone();

    let controllers = [controllers::web::get_router(cloned_pool)];
    let app = controllers.into_iter().fold(
        Router::new()
            .route("/", get(root))
            .route("/health", get(health_check))
            .route("/users", post(add_user))
            .with_state(pool.clone()),
        |app, c| app.merge(c),
    );
    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
