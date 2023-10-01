
use serde_json::Value;

use crate::msgot::msgot::Link;
pub fn link_analyze(link: &Value, link_collect: &mut Link) {
  match link.get("attributes") {
    Some(value) => {
      match value.get("href") {
        Some(href) => {
          let str = href.as_str().unwrap();
          link_collect.resource.push(str.clone().to_owned());
        }
        _ => {}
      }
    }
    _ => {}
  }
  println!("link name_list: {:?}", link);
}