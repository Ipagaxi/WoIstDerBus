
use axum::{
  Router,
  extract::{Json},
  routing::{post},
};

use serde_json::{json, Value};
use serde::Deserialize;

use crate::{aseag_com, send_http_requests, error};
use crate::util_json;


#[derive(Debug, Deserialize)]
pub struct BusRoutePayload {
  pub out_frwd: bool,
  pub out_date: Option<String>,
  pub out_time: Option<String>,
  pub arr_station: StationData,
  pub dep_station: StationData
}

#[derive(Debug, Deserialize)]
pub struct StationData {
  pub name: String,
  pub lid: String,
  #[serde(rename = "type")]
  pub station_type: String,
  pub ext_id: String,
  pub coord_x: u32,
  pub coord_y: u32
}

pub fn client_com_routes() -> Router {
  Router::new()
      .route("/bus_route", post(request_bus_routes))
}

async fn request_bus_routes(Json(payload): Json<BusRoutePayload>) -> error::Result<Json<Value>> {
  let url = "https://auskunft.avv.de/bin/mgate.exe?rnd=1739272765061";
  let body = aseag_com::construct_bus_route_request_body(payload);
  match send_http_requests::send_get_request(url, body).await {
    Ok(result) => {
      
      let response = Json(json!(
        util_json::get_infos_of_all_busses_for_route(&result)
      ));
      Ok(response)
    },
    Err(err) => {
      println!("Error: {}", err);
      Err(error::Error::RequestFail)
    }
  }
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
