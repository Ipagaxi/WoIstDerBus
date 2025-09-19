use axum::{
    Router
};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

pub use server::error::{Error, Result};

use server::client_com;

#[derive(Debug, Deserialize)]
struct Params {
    name: Option<String>
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let routes_all = Router::new()
        .merge(client_com::client_com_routes())
        .layer(cors);

    let url = "0.0.0.0";
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", url, port)).await.unwrap();
    println!("Server is running on http://{}:{}", url, port);
    axum::serve(listener, routes_all).await.unwrap();
}
