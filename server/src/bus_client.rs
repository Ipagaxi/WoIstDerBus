
use axum::{
  Router,
  extract::Json,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
};
use tokio::task;
use std::time::Duration;
use serde_json::{json, Value};

struct GetBusInfoParams {
  id: u32,
}

/*pub fn routes() -> Router {
  Router::new().route("/get-bus", post())

  

}*/

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