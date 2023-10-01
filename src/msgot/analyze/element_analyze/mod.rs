mod div;
mod meta;
mod script;
mod p;
mod link;
mod a;
mod img;
mod span;
mod rest;
use crate::msgot::msgot::Link;
use serde_json::Value;

pub fn element_analyze(element: &Value, link:&mut Link) {
  match element["name"].as_str() {
      Some("script") => {
        script::script_analyze(element);
      }
      Some("link") => {
        link::link_analyze(element, link);
      }
      Some("meta") => {
        meta::meta_analyze(element);
      }
      Some("div") => {
        div::div_analyze(element, link);
      }
      Some("p") => {
        p::p_analyze(element, link);
      }
      Some("span") => {
        span::span_analyze(element, link);
      }
      Some("a") => {
        a::a_analyze(element, link);
      }
      Some("img") => {
        img::img_analyze(element, link);
      }
      _ => {
        rest::rest_analyze(element, link);
      }
  }
}