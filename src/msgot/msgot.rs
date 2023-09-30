use std::fs;
use serde_json::Value;
// use crate::msgot::html_get::html_get;
use crate::msgot::analyze;
use serde::{Deserialize, Serialize};
// use crate::msgot::document_parse::document_parse;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    tar: Vec<String>
}

fn get_analyze_by_tar(tar: &str) {
  let html_name = &format!("{}.html", tar);
  let json_name = &format!("{}.json", tar);

  // 请求 html
  // html_get(tar, html_name)await;

  let html = fs::read_to_string(html_name).expect("file read error");

  // 转化数据结构
  // document_parse(&html, json_name);

  let json = fs::read_to_string(json_name).expect("file read error");
  let v: Value = serde_json::from_str(&json)?;
  // 解析
  analyze::page::analyze(&v);
}

pub async fn msgot () -> Result<(), Box<dyn std::error::Error>> {


  let json = fs::read_to_string("config.json").unwrap();
  let config: Config = serde_json::from_reader(&json)?;

  for va in config.tar.iter() {
    get_analyze_by_tar(va) 
  }

  Ok(())
}
