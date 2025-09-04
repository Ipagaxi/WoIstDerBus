use std::fs;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

use server::{util_json, aseag_com, client_com, send_http_requests, error};

#[cfg(test)]
mod tests {
  use axum::Json;
  use serde_json::json;

use super::*;

  #[test]
  fn test_get_value_from_json() {
    match fs::read_to_string("tests/BusDataJson.json") {
      Ok(json_data) => {
        let parsed_json: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");
        let path_string = vec!["svcResL[0]", "res", "outConL[0]", "secL[1]", "jny", "pos"];
        let result = util_json::get_value_by_path(&parsed_json, &path_string);
        println!("result: {:?}", result);
        assert_eq!(*result[0], json!({"x": 6063026, "y": 50769978}));
      },
      Err(why) => {
        println!("Could not read 'example.json'! ({:?})", why);
      },
    }
  }

  #[test]
  fn test_get_mult_value_from_json() {
    match fs::read_to_string("tests/BusDataJson.json") {
      Ok(json_data) => {
        let parsed_json: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");
        let json_path = vec!["svcResL", "res", "outConL", "secL", "jny", "pos"];
        let result = util_json::get_value_by_path(&parsed_json, &json_path);
        println!("result: {:?}", result);
        assert_eq!(*result[0], json!({"x": 6063026, "y": 50769978}));
        assert_eq!(*result[1], json!({"x": 6121609, "y": 50780199}));
      },
      Err(why) => {
        println!("Could not read 'example.json'! ({:?})", why);
      },
    }
  }

  #[tokio::test]
  async fn test_get_value_element_from_array_json() {
    let payload: client_com::BusRoutePayload = client_com::BusRoutePayload {
        out_frwd: true,
        out_date: None,
        out_time: None,
        arr_station: client_com::StationData {
          name: "Halifaxstraße".to_string(),
          lid: "A=1@O=Halifaxstraße@X=6058001@Y=50779534@U=80@L=1427@B=1@p=1739229385@".to_string(),
          station_type: "S".to_string(),
          ext_id: "1427".to_string(),
          coord_x: 6058001,
          coord_y: 50779534
        },
        dep_station: client_com::StationData {
          name: "Bushof, AC".to_string(),
          lid: "A=1@O=Bushof, AC@X=6089661@Y=50776477@U=80@L=1001@B=1@p=1740697285@".to_string(),
          station_type: "S".to_string(),
          ext_id: "1001".to_string(),
          coord_x: 6089661,
          coord_y: 50776477
        }
      };
    let url = "https://auskunft.avv.de/bin/mgate.exe?rnd=1739272765061";
    let body = aseag_com::construct_bus_route_request_body(payload);
    match send_http_requests::send_get_request(url, body).await {
      Ok(result) => {
        let response = Json(json!(
          util_json::get_infos_of_all_busses_for_route(&result)
        ));
        println!("Bus positions: {:?}", response);
      },
      Err(err) => {
        println!("Error: {}", err);
      }
    }
  }

}