
use axum::{
  Router,
  extract::{Json, Query},
  http::StatusCode,
  response::{IntoResponse, Html},
  routing::{get, post},
};
use tokio::task;
use std::time::Duration;
use serde_json::{json, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GetBusInfoParams {
  id: u32,
}

pub fn client_com_routes() -> Router {
  Router::new()
      .route("/bus_route", get(request_bus))
}

async fn request_bus(Query(params): Query<GetBusInfoParams>) -> impl IntoResponse {

  let name = params.id;
  Html(format!("Params: {name}"))
}

async fn get_bus_info(Json(params): Json<GetBusInfoParams>) -> Json<Value> {
  println!("Received Params: ID = {}", params.id);

  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  task::spawn(async {
    for i in 1..=10 {
        println!("Background task running: iteration {}", i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("Background task completed!");
  });

  body
}