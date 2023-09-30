mod div;
mod meta;
mod script;
mod link;

use serde_json::Value;

pub fn element_analyze(element: &Value) {
  match element["name"].as_str() {
      Some("script") => {
        script::script_analyze(element);
      }
      Some("link") => {
        link::link_analyze(element);
      }
      Some("meta") => {
        meta::meta_analyze(element);
      }
      Some("div") => {
        div::div_analyze(element);
      }
      _ => {
        println!("{:?}", element["name"])
      }
  }
}