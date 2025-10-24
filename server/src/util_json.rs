use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusData {
  pub name: String,
  pub direction_text: String,
  pub pos: BusPosition,
  pub dep_time: u16,
  pub arr_time: u16,
  pub poly_line: String
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

fn get_encoded_poly_lines(routes_data_value: &Value) -> Vec<String> {
  let json_poly_l_path = vec!["svcResL", "res", "common", "polyL", "crdEncYX"];
  let encoded_poly_lines_value = get_value_by_path(&routes_data_value, &json_poly_l_path);
  let mut encoded_poly_lines_string: Vec<String> = Vec::new();
  for poly in encoded_poly_lines_value {
    encoded_poly_lines_string.push(poly.as_str().unwrap_or("").to_string());
  }
  encoded_poly_lines_string
}

pub fn get_infos_of_all_busses_for_route(routes_data_json: &str) -> Vec<BusData> {
  let routes_data_value: Value = serde_json::from_str(&routes_data_json).expect("Failed to parse JSON");

  let mut bus_data_vec: Vec<BusData> = Vec::new();
  let json_jiny_l_path = vec!["svcResL", "res", "outConL", "secL", "jny", "freq", "jnyL"];
  let value_jny_l_vec = get_value_by_path(&routes_data_value, &json_jiny_l_path);
  let encoded_poly_lines = get_encoded_poly_lines(&routes_data_value);
  for (i, jny_l_value) in value_jny_l_vec.iter().enumerate() {
    if let Some(arr) = jny_l_value.as_array() {
      for entry in arr {
        // parse bus name/number e.g. "33" an departure and arrival time
        let (name, dep_time, arr_time) = parse_bus_name_and_dep_arr_time(entry);
        // parse bus direction text, e.g. "Uniklinik"
        let direction = parse_bus_direction(entry);
        let mut pos = BusPosition { x: 0, y: 0 };
        if let Some(pos_value) = entry.get("pos") {
          pos = cast_pos_value_to_struct(pos_value.clone());
        }
        let bus_data = BusData {
          name: name.to_string(),
          direction_text: direction.to_string(),
          pos,
          dep_time,
          arr_time,
          poly_line: encoded_poly_lines[i].clone()
        };
        bus_data_vec.push(bus_data);
      }
    }
  }
  let example_bus_data = BusData {
    name: "66".to_string(),
    direction_text: "XY".to_string(),
    pos: BusPosition { x: 6189221, y: 51777163 },
    dep_time: 1200,
    arr_time: 1220,
    poly_line: "_h|tHkkdd@gCvADRg@n@MLoBdCy@|Bg@xIMC??LBGbAe@|FkB~JCnBl@lBLJXbAGF??FGVz@bArDZ`@kEpFi@b@gE@Ac@oB_Dk@|@AM??@L_@`B{@p@Bb@h@`BtD~Hh@j@G^??F_@|An@k@lDmEnIZ`@j@bAx@T?T???U\\Lz@tA\\`AFhAkBzKWt@ENIE??HD[v@GVl@r@PDpCbIhCjFrBbF~@pE\\bDID??HEH`EOrDS|AEhBaB`@uBvAi@l@EU]x@".to_string()
  };
  bus_data_vec.push(example_bus_data);
  bus_data_vec
}