#![allow(clippy::needless_return, clippy::manual_let_else)]

mod helpers;
mod models;
mod repositories;
mod routes;
use axum::Router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DB_FILE").expect("DB_FILE must be set");
    let pool = helpers::db::pre_load_db_into_memory(database_url).await;

    let app_state = helpers::AppState { pool };

    let app = Router::new()
        .nest("/card", routes::card::router())
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind to addr");
    println!("listening on port {port}");
    axum::serve(listener, app)
        .await
        .expect("server failed to start");
}
