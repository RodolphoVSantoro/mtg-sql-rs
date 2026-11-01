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
    let db_mode = std::env::var("DB_MODE").expect("DB_MODE must be set");

    let pool = match db_mode.as_str() {
        "memory" => helpers::db::pre_load_db_into_memory(database_url).await,
        "disk" =>helpers::db::create_pool_to_disk_file(database_url).await,
        _ => panic!("Unknown db mode '{db_mode}', allowed types: [\"memory\", \"disk\"]"),
    };

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
