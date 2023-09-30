use serde_json::Value;

pub fn meta_analyze(meta: &Value) {
  println!("meta name_list: {:?}", meta);
}