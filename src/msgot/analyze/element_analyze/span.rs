use serde_json::Value;

use crate::msgot::msgot::Link;

pub fn span_analyze(span: &Value, link: &mut Link) {
  match span.get("children") {
    Some(children) => {
      for val in children.as_array().unwrap().iter() {
        if val.is_string() {
          link.span.push(val.as_str().unwrap().clone().to_owned())
        }
      }
    }
    _ => {}
  }
}