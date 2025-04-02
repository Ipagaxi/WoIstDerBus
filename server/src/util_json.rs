use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusPosition {
  x: f32,
  y: f32
}

pub fn get_infos_of_all_busses_for_route(json_data: &str) -> Vec<BusPosition> {

    let parsed_json: Value = serde_json::from_str(json_data).expect("Failed to parse JSON");

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
                if let Some(jiny_l_array) = sec_l.get("jny")
                  .and_then(|jny| jny.get("freq"))
                  .and_then(|freq| freq.get("JinyL"))
                  .and_then(|jiny_l_array| jiny_l_array.as_array())
                {
                  let positions: Vec<BusPosition> = jiny_l_array.iter()
                                           .filter_map(|jiny_l| jiny_l.get("pos").and_then(|pos| serde_json::from_value(pos.clone()).ok()))
                                           .collect();
                  println!("Got positions of {} busses", positions.len());
                  println!("positions: {:?}", positions);
                  return positions;
                } else {
                  println!("No jny found!");
                }
              }
            } else {
              println!("No secL found!");
            }
          }
        } else {
          println!("No res found!");
        }
      }
    } else {
        println!("No svcResL found!");
    }
    println!("No bus position found!");
    return Vec::new();
}