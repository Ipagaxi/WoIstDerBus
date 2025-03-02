
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

use crate::{aseag_com, send_get_request, Error, Result};


#[derive(Debug, Deserialize)]
struct BusRoutePayload {
  out_frwd: bool,
  out_date: Option<String>,
  out_time: Option<String>,
  arr_station: StationData,
  des_station: StationData
}

#[derive(Debug, Deserialize)]
struct StationData {
  name: String,
  lid: String,
  #[serde(rename = "type")]
  station_type: String,
  ext_id: String
}

pub fn client_com_routes() -> Router {
  Router::new()
      .route("/bus_route", post(request_bus_routes))
}

async fn request_bus_routes(payload: Json<BusRoutePayload>) -> Result<Json<Value>> {
  /*match aseag_com::load_template_request_body() {
    Ok(body) => {
      let url = "https://auskunft.avv.de/bin/mgate.exe?rnd=1739272765061";
      match send_get_request(url, body).await {
        Ok(result) => println!("Result: {:?}", result),
        Err(err) => println!("Error: {}", err),
    }
    },
    Err(err) => {
      println!("{}", err);
    }
  }*/

  let arr_name = payload.arr_station.name.clone();
  let des_name = payload.des_station.name.clone();

  let body = Json(json!({
    "response": {
      "success": true,
      "arr_name": arr_name,
      "des_name": des_name
    }
  }));
  Ok(body)
}

/*async fn get_bus_info(Json(params): Json<GetBusInfoParams>) -> Json<Value> {
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
}*/