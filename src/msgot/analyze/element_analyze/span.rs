use serde_json::Value;

pub fn span_analyze(span: &Value) {
  println!("span name_list: {:?}", span["name"]);
}