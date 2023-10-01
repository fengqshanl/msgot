use serde_json::Value;
use crate::msgot::msgot::Link;

use super::link;

pub fn a_analyze(a: &Value, link: &mut Link) {
  match a.get("attributes") {
    Some(value) => {
      match value.get("href") {
        Some(href) => {
          let str = href.as_str().unwrap();
          link.subdomain.push(str.clone().to_owned());
        }
        _ => {}
      }
    }
    _ => {}
  }
}