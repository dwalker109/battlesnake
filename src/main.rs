use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_get))
        .route("/start", post(handle_start))
        .route("/move", post(handle_move))
        .route("/end", post(handle_end));

    axum::Server::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 3001)))
        .serve(app.into_make_service())
        .await
        .expect("server is running");
}

async fn handle_get() -> &'static str {
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
