use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() {
    println!("Loading environment variables...");
    dotenv().ok();

    let app = Router::new().route("/", get(echo_status));

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("12007".to_string());
    let addr = format!("{}:{}", host, port);

    println!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn echo_status(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    println!("GET /echo_status");
    let default: String = "200".to_string();
    let status: u16 = params.get("status").unwrap_or(&default).parse().unwrap();
    println!("GET /echo_status::Received status {}", status);

    Response::builder()
        .status(status)
        .body(status.to_string())
        .unwrap()
}
