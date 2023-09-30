use serde::de::value::BoolDeserializer;
use serde_json::Value;
use crate::msgot::analyze::element_analyze::element_analyze;

fn iter_analyzer_array (elements: &Value) {
  element_analyze(elements);
  let template = elements["children"].clone(); 
  if !template.is_null() {
    for ele in template.as_array().unwrap().iter() {
      iter_analyzer_array(ele)
    } 
  }
}

pub fn analyze<'a>(html_obj: &Value) {
  println!("document analyzer start");
  let first_level = html_obj["children"].clone();
  iter_analyzer_array(&first_level); 
}