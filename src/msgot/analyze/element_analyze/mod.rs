mod div;
mod meta;
mod script;
mod p;
mod link;
mod a;
mod span;

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
      Some("p") => {
        p::p_analyze(element);
      }
      Some("span") => {     
        span::span_analyze(element);
      }
      Some("a") => {     
        a::a_analyze(element);
      }
      _ => {
        println!("none: {:?}", element)
      }
  }
}