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

  let json_path = vec!["svcResL", "res", "outConL", "secL", "jny", "pos"];
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

pub fn get_infos_of_all_busses_for_route_(json_data: &str) -> Vec<BusPosition> {

    let parsed_json: Value = serde_json::from_str(json_data).expect("Failed to parse JSON");
    let mut positions: Vec<BusPosition> = Vec::new();

    if let Some(svc_res_l_array) = parsed_json.get("svcResL")
                                        .and_then(|svc_res_l_array| svc_res_l_array.as_array()) 
    {
      for svc_res_l in svc_res_l_array.iter() {
        if let Some(out_con_l_array) = svc_res_l.get("res")
                  .and_then(|res| res.get("outConL"))
                  .and_then(|out_con_l_array| out_con_l_array.as_array())
        {
          for out_con_l in out_con_l_array.iter() {
            if let Some(sec_l_array) = out_con_l.get("secL")
              .and_then(|sec_l_array| sec_l_array.as_array())
            {
              for sec_l in sec_l_array.iter() {
                if let Some(pos) = sec_l.get("jny")
                  .and_then(|jny| {
                    jny.get("pos").and_then(|pos| serde_json::from_value(pos.clone()).ok())
                  }) {
                  positions.push(pos);
                } 
                else if let Some(jny_l_array) = sec_l.get("jny")
                  .and_then(|jny| {
                    jny.get("freq")
                  })
                  .and_then(|freq| {
                    freq.get("jnyL")
                  })
                  .and_then(|jny_l_array| {
                    jny_l_array.as_array()
                  })
                {
                  positions.extend(jny_l_array.iter()
                                           .filter_map(|jny_l| {
                                             jny_l.get("pos").and_then(|pos| serde_json::from_value(pos.clone()).ok())
                                           })
                                           .collect::<Vec<BusPosition>>());
                } else {
                }
              }
            } else {
            }
          }
        } else {
        }
      }
    } else {
    }
    positions.push(BusPosition { x: 6189221, y: 51777163 });
    return positions;
}
