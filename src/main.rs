use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use battlesnake_game_types::{
    compact_representation::StandardCellBoard4Snakes11x11,
    types::{build_snake_id_map, Move, YouDeterminableGame},
    wire_representation::Game,
};
use battlesnake_minimax::{MinimaxReturn, ParanoidSnake};
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

#[instrument(skip(wire_game), fields(game_id = %wire_game.game.id))]
async fn make_move(Json(wire_game): Json<Game>) -> impl IntoResponse {
    info!("Hit Make Move Route");

    let game_info = wire_game.game.clone();
    let turn = wire_game.turn;

    let snake_id_map = build_snake_id_map(&wire_game);
    let compact_game =
        StandardCellBoard4Snakes11x11::convert_from_game(wire_game, &snake_id_map).unwrap();

    let minimax_snake = ParanoidSnake::from_func(&score_function, "minimax_snake");

    // Now we can use the minimax snake to generate the next move!
    // Here we use the function deepend_minimax to run the minimax algorithm for 100 milliseconds
    // before returning the best move
    let result: MinimaxReturn<_, _> = minimax_snake.deepened_minimax(
        compact_game,
        game_info,
        turn,
        std::time::Duration::from_millis(300),
    );

    let chosen_move = result.direction_for(compact_game.you_id()).unwrap();

    Json(json!({ "move": chosen_move.to_string() }))
}

// This is the scoring function that we will use to evaluate the game states
// Here it just returns a constant but would ideally contain some logic to decide which
// states are better than others
fn score_function(_board: &StandardCellBoard4Snakes11x11) -> i32 {
    4
}
