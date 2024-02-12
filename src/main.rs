use axum::{
    routing::get,
    Router,
};
use std::env;

#[tokio::main]
async fn main() {

    let mut healthcheck_path = String::from("/healthz");
    let healthcheck_override = env::var("HEALTHCHECK_PATH").is_ok();
    if healthcheck_override {
        healthcheck_path = env::var("HEALTHCHECK_PATH").unwrap();
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(&healthcheck_path, get(|| async {""}));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}