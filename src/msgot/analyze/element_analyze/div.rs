use serde_json::Value;

use crate::msgot::msgot::Link;

pub fn div_analyze(div: &Value, link: &mut Link) {
  match div.get("children") {
    Some(children) => {
      for val in children.as_array().unwrap().iter() {
        if val.is_string() {
          link.div.push(val.as_str().unwrap().clone().to_owned())
        }
      }
    }
    _ => {}
  }
}