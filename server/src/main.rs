use axum::{
    handler, response::{Html, IntoResponse}, routing::{get, post}, Router
};

pub mod bus_client;

use crate::bus_client::*;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler_root))
        .route("/get-bus", post(handler_root));

    // run our app with hyper, listening globally on port 3000
    let url = "0.0.0.0";
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", url, port)).await.unwrap();
    println!("Server is running on http://{}:{}", url, port);
    axum::serve(listener, app).await.unwrap();
}

async fn handler_root() -> impl IntoResponse {
    Html("hello, world!")
}