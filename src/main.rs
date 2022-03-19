use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use battlesnake_game_types::{types::Move, wire_representation::Game};
use serde::Serialize;
use serde_json::json;
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
        .route("/", get(snake_info))
        .route("/start", post(game_start))
        .route("/end", post(game_end))
        .route("/move", post(make_move));

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
    info!("Hit Snake Info Route");

    Json(SnakeInfo {
        apiversion: "1".to_string(),
        author: None,
        color: None,
        head: None,
        tail: None,
        version: None,
    })
}

#[instrument(skip(game), fields(game_id = %game.game.id))]
async fn game_start(Json(game): Json<Game>) -> impl IntoResponse {
    info!("Hit Start Route");

    StatusCode::OK
}

#[instrument(skip(game), fields(game_id = %game.game.id))]
async fn game_end(Json(game): Json<Game>) -> impl IntoResponse {
    info!("Hit End Route");

    StatusCode::OK
}

#[instrument(skip(game), fields(game_id = %game.game.id))]
async fn make_move(Json(game): Json<Game>) -> impl IntoResponse {
    info!("Hit Make Move Route");

    Json(json!({ "move": Move::Right.to_string() }))
}
