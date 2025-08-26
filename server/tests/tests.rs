use std::fs;
use serde_json::Value;

use server::util_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(1+1, 2);
  }

  #[test]
  fn test_get_value_from_json() {
    match fs::read_to_string("tests/example.json") {
      Ok(json_data) => {
        let parsed_json: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");
        let path_string = "client.id";
        let result = util_json::get_value_by_path(&parsed_json, path_string);
        assert_eq!(result, Some(Value::String("AVV_AACHEN".to_string())).as_ref());
      },
      Err(why) => {
        println!("Could not read 'example.json'! ({:?})", why);
      },
    }
  }

}