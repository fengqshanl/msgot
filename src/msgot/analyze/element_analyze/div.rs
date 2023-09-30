use serde_json::Value;

pub fn div_analyze(div: &Value) {
  println!("div name_list: {:?}", div["name"]);
}