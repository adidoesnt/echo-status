use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    println!("Loading environment variables...");
    dotenv().ok();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("12007".to_string());
    let addr = format!("{}:{}", host, port);

    println!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
