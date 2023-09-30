mod div;
mod meta;
mod script;
mod link;

use serde_json::Value;

pub fn element_analyze(element: &Value) {
  match element["name"].as_str() {
      Sone("script") => {
        script::script_analyze(element);
      }
      Some("link") => {
        link::link_analyze(element);
      }
      Sone("meta") => {
        meta::meta_analyze(element);
      }
      Sone("div") => {
        div::div_analyze(element);
      }
      _ => {
        println!("{:?}", element["name"])
      }
  }
}