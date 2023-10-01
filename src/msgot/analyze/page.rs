use serde_json::Value;
use crate::msgot::{analyze::element_analyze::element_analyze, msgot::Link};

fn iter_analyzer_array (elements: &Value, link:&mut Link) {
  element_analyze(elements, link);
  match elements.get("children") {
    Some(template) => {
      for ele in template.as_array().unwrap().iter() {
        iter_analyzer_array(ele, link)
      } 
    }
    _ => {}
  }
}

pub fn analyze<'a>(html_obj: &Value, link:&mut Link) {
  iter_analyzer_array(html_obj, link); 
}