use serde_json::Value;

pub fn p_analyze(p: &Value) {
  println!("p name_list: {:?}", p["name"]);
}