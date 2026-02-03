use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod models;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the app route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health));

    // Run app on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
