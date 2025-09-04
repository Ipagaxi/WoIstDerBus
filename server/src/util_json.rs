use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)] 
pub struct BusPosition {
  x: u32,
  y: u32
}

pub fn get_value_by_path<'a>(json_data: &'a Value, path: &Vec<&str>) -> Vec<&'a Value> {
    let mut current = json_data;
    let mut positions: Vec<&Value> = Vec::new();
    for mut i in 0..path.len() {
        // Check if the segment contains array indexing like "foo[2]"
        if let Some((key, index_str)) = &path[i].split_once('[') {
            // Navigate into the object
            match current.get(key) {
              Some(value) => {
                current = value;
              },
              None => {
                return positions
              }
            }
            // Navigate into the array
            let index = index_str.trim_end_matches(']').parse::<usize>().ok();
            match current.get(index.unwrap_or(0)) {
              Some (value) => {
                current = value;
              },
              None => {
                return positions
              }
            }
        } else {
            match current.get(&path[i]) {
              Some(value) => {
                // If an index is not specified => go through every
                if i+1 < path.len() {
                  if let Some(arr) = value.as_array() {
                    for element in arr {
                      positions.append(&mut get_value_by_path(element, &path[i+1..].to_vec()));
                    }
                    i = path.len();

                  } else {
                    // Just a regular key
                    current = value;
                  } 
                } else {
                  current = value;
                }
              },
              None => {
                return positions
              }
            }
        }
    }
    positions.push(current);
    positions
}

pub fn get_infos_of_all_busses_for_route(route_data_json: &str) -> Vec<BusPosition> {
  let route_data_value: Value = serde_json::from_str(route_data_json).expect("Failed to parse JSON");

  let json_path = vec!["svcResL", "res", "outConL", "secL", "jny", "freq", "jnyL", "pos"];
  let value_bus_positions = get_value_by_path(&route_data_value, &json_path);
  let mut bus_positions: Vec<BusPosition> = Vec::new();
  for value_position in value_bus_positions {
    match serde_json::from_value(value_position.clone()) {
      Ok(position) => {
        bus_positions.push(position);
      },
      Err(e) => {
        println!("Error: {}", e);
      },
    }
  }
  bus_positions
}