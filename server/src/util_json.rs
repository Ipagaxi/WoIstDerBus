use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusData {
  name: String,
  direction_text: String,
  pos: BusPosition
}

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

fn get_bus_name(info_string: &str) -> &str {
  // hay example: "T$A=1@O=Aachen, Bushof@L=1001@a=128@$A=1@O=Aachen, Halifaxstraße@L=1427@a=128@$202509041222$202509041232$Bus   73$$1$$$$$$"
  let re = Regex::new(r"\$Bus\s*([^\$]+)").unwrap();
  if let Some(caps) = re.captures(info_string) {
        caps.get(1).map_or("None", |m| &info_string[m.start()..m.end()])
  } else { 
    println!("No regex captures found");
    "None"
  }
}

fn cast_pos_value_to_struct(pos_value: Value) -> BusPosition {
  match serde_json::from_value(pos_value) {
    Ok(pos) => pos,
    Err(e) => {
      println!("Error: {}", e);
      BusPosition {
        x: 0,
        y: 0
      }
    }
  }
}

fn parse_bus_name(json_property_value: &Value) -> &str {
  if let Some(info_value) = json_property_value.get("ctxRecon") {
    match info_value.as_str() {
      Some(info_str) => {
        get_bus_name(info_str)
      },
      None => {
        println!("Property ctxRecon found, but it is not a string?!");
        "None"
      }
    }
  } else {
    println!("No info string with bus name found (ctxRecon)");
    "None"
  }
}

fn parse_bus_direction(json_property_value: &Value) -> &str {
  if let Some(direction_value) = json_property_value.get("dirTxt") {
    match direction_value.as_str() {
      Some(direction_str) => direction_str,
      None => {
        println!("Property 'dirTxt' found, but it is not a string?!");
        "N/A"
      }
    }
  } else {
    println!("No direction text found (dirTxt)!");
    "N/A"
  }
}

pub fn get_infos_of_all_busses_for_route(route_data_json: &str) -> Vec<BusData> {
  let route_data_value: Value = serde_json::from_str(route_data_json).expect("Failed to parse JSON");

  let mut bus_data_vec: Vec<BusData> = Vec::new();
  let json_jiny_l_path = vec!["svcResL", "res", "outConL", "secL", "jny", "freq", "jnyL"];
  let value_jny_l_vec = get_value_by_path(&route_data_value, &json_jiny_l_path);
  for jny_l_value in value_jny_l_vec {
    if let Some(arr) = jny_l_value.as_array() {
      for entry in arr {
        // Only parse busses that have a live position
        if let Some(pos_value) = entry.get("pos") {
          let pos = cast_pos_value_to_struct(pos_value.clone());
          
          // parse bus name/number e.g. "33"
          let name = parse_bus_name(&entry);
          // parse bus direction text, e.g. "Uniklinik"
          let direction = parse_bus_direction(&entry);
          let bus_data = BusData {
            name: name.to_string(),
            direction_text: direction.to_string(),
            pos
          };
          bus_data_vec.push(bus_data);
        }
      }
    }
  }
  bus_data_vec

  /*
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
  */
}