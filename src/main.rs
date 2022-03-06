use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{rejection::JsonRejection, Extension, Json},
    response::IntoResponse,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use battlesnake_server::snake::{Dir, Webhooks, SNAKE_VITALS};
use battlesnake_server::{
    api::{Battlesnake, Board, Game},
    snake,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{debug, info};

type SnakeMap = HashMap<String, Box<dyn Webhooks + Send + Sync>>;
type SharedSnakeMap = Arc<RwLock<SnakeMap>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let games: SnakeMap = HashMap::new();

    let app = Router::new()
        .route("/", get(handle_get))
        .route("/start", post(handle_start))
        .route("/move", post(handle_move))
        .route("/end", post(handle_end))
        .layer(AddExtensionLayer::new(Arc::new(RwLock::new(games))));

    axum::Server::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 3001)))
        .serve(app.into_make_service())
        .await
        .expect("server is running");
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Payload {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

async fn handle_get() -> impl IntoResponse {
    Json(SNAKE_VITALS)
}

async fn handle_start(
    Extension(games): Extension<SharedSnakeMap>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    debug!("hit start");
    debug!("{:?}", payload);
    let snake = snake::starter::Starter;
    games
        .write()
        .unwrap()
        .insert(payload.game.id, Box::new(snake));
}

async fn handle_move(Extension(games): Extension<SharedSnakeMap>) -> impl IntoResponse {
    Json(json!({ "move": Dir::UP }))
}

async fn handle_end(Extension(games): Extension<SharedSnakeMap>) -> impl IntoResponse {}
