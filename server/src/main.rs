use axum::{
    extract::Query, handler, response::{Html, IntoResponse}, routing::{get, post}, Router
};
use serde::Deserialize;

pub mod client_com;
pub mod aseag_com;
pub mod send_http_requests;

use crate::client_com::*;
use crate::aseag_com::*;
use crate::send_http_requests::*;

#[derive(Debug, Deserialize)]
struct Params {
    name: Option<String>
}

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(client_com_routes());

    let url = "0.0.0.0";
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", url, port)).await.unwrap();
    println!("Server is running on http://{}:{}", url, port);
    axum::serve(listener, routes_all).await.unwrap();
}