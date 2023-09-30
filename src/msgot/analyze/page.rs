use serde_json::Value;
use crate::msgot::analyze::element_analyze::element_analyze;

fn iter_analyzer_array (elements: &Value) {
  element_analyze(elements);
  match elements.get("children") {
    Some(template) => {
      for ele in template.as_array().unwrap().iter() {
        iter_analyzer_array(ele)
      } 
    }
    _ => {}
  }
}

pub fn analyze<'a>(html_obj: &Value) {
  iter_analyzer_array(html_obj); 
}