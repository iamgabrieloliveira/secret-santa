use crate::routes::{add_participants, generate_links, reveal_gift};
use crate::state::AppState;
use axum::{routing::get, routing::post, Router};
use routes::clear;
use std::net::SocketAddr;
use std::sync::Arc;

mod models;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(add_participants::form))
        .route("/clear", get(clear::clear))
        .route("/submit", post(add_participants::submit))
        .route("/generate-links", get(generate_links::generate))
        .route("/reveal/:id", get(reveal_gift::reveal))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
