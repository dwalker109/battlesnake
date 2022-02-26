use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::Extension,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use battlesnake_server::stable::Webhooks;

type SnakeMap = HashMap<String, Box<dyn Webhooks + Send + Sync>>;

#[tokio::main]
async fn main() {
    let games: SnakeMap = HashMap::new();

    let app = Router::new()
        .layer(AddExtensionLayer::new(Arc::new(games)))
        .route("/", get(handle_get))
        .route("/start", post(handle_start))
        .route("/move", post(handle_move))
        .route("/end", post(handle_end));

    axum::Server::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 3001)))
        .serve(app.into_make_service())
        .await
        .expect("server is running");
}

async fn handle_get(Extension(games): Extension<Arc<SnakeMap>>) -> &'static str {
    "OK"
}

async fn handle_start() -> &'static str {
    "OK"
}

async fn handle_move() -> &'static str {
    "OK"
}

async fn handle_end() -> &'static str {
    "OK"
}
