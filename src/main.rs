#![allow(clippy::all)]

mod models;
mod state;
mod routers;
mod handlers;

use crate::{ routers::app, state::AppState };

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Task system running on https://localhost:8000");
    axum::serve(listener, app(state)).await.unwrap();
}
