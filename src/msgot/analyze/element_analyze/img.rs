use serde_json::Value;
use crate::msgot::msgot::{ Link, BasicLink };

pub fn img_analyze(img: &Value, link: &mut Link) {
  match img.get("attributes") {
    Some(value) => {
      let src = value.get("src");
      let alt = value.get("alt");
      if src.is_some() {
        let mut alt_val = String::new();
        if alt.is_some() {
          alt_val = alt.unwrap().as_str().unwrap().clone().to_owned(); 
        }
        link.img.push(BasicLink {
          text: alt_val.clone(),
          url: src.unwrap().as_str().unwrap().clone().to_owned()
        })
      }
    }
    _ => {}
  }
}