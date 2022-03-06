use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use battlesnake_server::{
    api::{GetResponse, MoveResponse, Payload},
    snake::{self, Dir, Webhooks},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type SnakeMap = HashMap<(String, String), Box<dyn Webhooks + Send + Sync>>;
type SharedSnakeMap = Arc<RwLock<SnakeMap>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "battlesnake_server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let games: SnakeMap = HashMap::new();

    let app = Router::new()
        .route("/", get(handle_get))
        .route("/start", post(handle_start))
        .route("/move", post(handle_move))
        .route("/end", post(handle_end))
        .layer(AddExtensionLayer::new(Arc::new(RwLock::new(games))));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on port {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server is running");
}

async fn handle_get() -> impl IntoResponse {
    tracing::debug!("hit /get");

    Json(GetResponse {
        apiversion: "1",
        author: "Dan Walker",
        color: "#E80978",
        head: "default",
        tail: "curled",
        version: "0.0.1",
    })
}

async fn handle_start(
    Extension(games): Extension<SharedSnakeMap>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    tracing::debug!("hit /start with payload {:?}", &payload);

    let snake = snake::starter::Starter;

    games
        .write()
        .unwrap()
        .insert((payload.game.id, payload.you.id), Box::new(snake));

    tracing::debug!("active games: {}", games.read().unwrap().iter().count());
}

async fn handle_move(Extension(games): Extension<SharedSnakeMap>) -> impl IntoResponse {
    tracing::debug!("hit /move");
    tracing::debug!("active games: {}", games.read().unwrap().iter().count());

    Json(MoveResponse::from(Dir::DOWN))
}

async fn handle_end(
    Extension(games): Extension<SharedSnakeMap>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    tracing::debug!("hit /end with payload {:?}", &payload);

    games
        .write()
        .unwrap()
        .remove(&(payload.game.id, payload.you.id));

    tracing::debug!("active games: {}", games.read().unwrap().iter().count());
}
