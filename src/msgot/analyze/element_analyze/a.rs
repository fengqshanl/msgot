use serde_json::Value;
use crate::msgot::msgot::{ Link, BasicLink };

use super::link;

pub fn a_analyze(a: &Value, link: &mut Link) {
  match a.get("attributes") {
    Some(value) => {
      match value.get("href") {
        Some(href) => {
          let str = href.as_str().unwrap();
          let mut text:Vec<String> = Vec::new();
          let val = a.get("children");
          if val.is_some() {
            for v in val.unwrap().as_array().unwrap().iter() {
              if v.is_string() {
                text.push(v.as_str().unwrap().to_owned())
              }
            }
          }
          link.subdomain.push(BasicLink {
            text: Into::<String>::into(text.join(",")).clone(),
            url: str.clone().to_owned()
          });
        }
        _ => {}
      }
    }
    _ => {}
  }
}