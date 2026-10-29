mod helpers;
mod models;
mod routes;
use axum::Router;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();

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
