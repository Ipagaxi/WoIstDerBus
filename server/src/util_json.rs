use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusData {
  pub name: String,
  pub direction_text: String,
  pub pos: BusPosition,
  pub dep_time: u16,
  pub arr_time: u16
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct BusPosition {
  x: u32,
  y: u32
}

pub fn get_value_by_path<'a>(json_data: &'a Value, path: &[&str]) -> Vec<&'a Value> {
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

fn regex_bus_name(info_string: &str) -> &str {
  // hay example: "T$A=1@O=Aachen, Bushof@L=1001@a=128@$A=1@O=Aachen, Halifaxstraße@L=1427@a=128@$202509041222$202509041232$Bus   73$$1$$$$$$"
  let re = Regex::new(r"\$Bus\s*([^\$]+)").unwrap();
  if let Some(caps) = re.captures(info_string) {
        caps.get(1).map_or("None", |m| &info_string[m.start()..m.end()])
  } else { 
    println!("No regex capture for bus name found");
    "None"
  }
}

fn regex_bus_dep_and_arr_time(info_string: &str) -> (u16, u16) {
  let re = Regex::new(r"\$(\d{8})(\d{4})\$(\d{8})(\d{4})").unwrap();
  if let Some(caps) = re.captures(info_string) {
    let dep_time = caps.get(2).map_or("None", |m| &info_string[m.start()..m.end()]).parse::<u16>().unwrap();
    let arr_time = caps.get(4).map_or("None", |m| &info_string[m.start()..m.end()]).parse::<u16>().unwrap();
    (dep_time, arr_time)
  } else {
    println!("No regex capture for dep or arr time found");
    (0, 0)
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

fn parse_bus_name_and_dep_arr_time(json_property_value: &Value) -> (&str, u16, u16) {
  if let Some(info_value) = json_property_value.get("ctxRecon") {
    match info_value.as_str() {
      Some(info_str) => {
        let name = regex_bus_name(info_str);
        let (dep, arr) = regex_bus_dep_and_arr_time(info_str);
        (name, dep, arr)
      },
      None => {
        println!("Property ctxRecon found, but it is not a string?!");
        ("N/A", 0, 0)
      }
    }
  } else {
    println!("No info string with bus name found (ctxRecon)");
    ("N/A", 0, 0)
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
          let (name, dep_time, arr_time) = parse_bus_name_and_dep_arr_time(entry);
          // parse bus direction text, e.g. "Uniklinik"
          let direction = parse_bus_direction(entry);
          let bus_data = BusData {
            name: name.to_string(),
            direction_text: direction.to_string(),
            pos,
            dep_time,
            arr_time
          };
          bus_data_vec.push(bus_data);
        }
      }
    }
  }
  let example_bus_data = BusData {
    name: "66".to_string(),
    direction_text: "XY".to_string(),
    pos: BusPosition { x: 6189221, y: 51777163 },
    dep_time: 1200,
    arr_time: 1220
  };
  bus_data_vec.push(example_bus_data);
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