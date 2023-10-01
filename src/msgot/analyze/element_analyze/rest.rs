use serde_json::Value;

use crate::msgot::msgot::Link;

pub fn rest_analyze(rest: &Value, link: &mut Link) {
  if rest.is_string() { 
    link.span.push(rest.as_str().unwrap().clone().to_owned())
  }
}