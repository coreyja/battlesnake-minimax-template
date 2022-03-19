use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::{env, net::SocketAddr};
use tracing::{info, info_span, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Serialize)]
pub struct SnakeInfo {
    apiversion: String,
    author: Option<String>,
    color: Option<String>,
    head: Option<String>,
    tail: Option<String>,
    version: Option<String>,
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(snake_info));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument]
async fn snake_info() -> impl IntoResponse {
    Json(SnakeInfo {
        apiversion: "1.0".to_string(),
        author: None,
        color: None,
        head: None,
        tail: None,
        version: None,
    })
}
