use serde_json::Value;

pub fn a_analyze(a: &Value) {
  println!("a name_list: {:?}", a["name"]);
}