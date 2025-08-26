use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)] 
pub struct BusPosition {
  x: u32,
  y: u32
}

pub fn get_value_by_path<'a>(json_data: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = json_data;
    for segment in path.split('.') {
        // Check if the segment contains array indexing like "foo[2]"
        if let Some((key, index_str)) = segment.split_once('[') {
            // Navigate into the object
            match current.get(key) {
              Some(value) => {
                println!("Value: {:?}", value);
                current = value;
              },
              None => {
                return None
              }
            }
            // Navigate into the array
            let index = index_str.trim_end_matches(']').parse::<usize>().ok();
            match current.get(index.unwrap_or(0)) {
              Some (value) => {
                println!("Value: {:?}", value);
                current = value;
              },
              None => {
                return None
              }
            }
        } else {
            // Just a regular key
            match current.get(segment) {
              Some(value) => {
                println!("Value: {:?}", value);
                current = value;
              },
              None => {
                return None
              }
            }
        }
    }
    Some(current)
}

pub fn get_infos_of_all_busses_for_route(json_data: &str) -> Vec<BusPosition> {

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
