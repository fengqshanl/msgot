use std::fs;
use std::io::Write;
use serde_json::json;
use serde_json::Value;
// use crate::msgot::html_get::html_get;
use crate::msgot::analyze;
use serde::{Deserialize, Serialize};
use crate::msgot::dispose::{dispose_focus, WholeConfig};
// use crate::msgot::document_parse::document_parse;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    tar: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicLink {
  pub text: String,
  pub url: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Link {
  // 子域名
  pub subdomain: Vec<BasicLink>,
  pub resource: Vec<String>,
  pub img: Vec<BasicLink>,
  pub span: Vec<String>,
  pub div: Vec<String>
}

fn get_analyze_by_tar(tar: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut link: Link = Link::default();
  let html_name = &format!("{}.html", tar);
  let json_name = &format!("{}.json", tar);

  // 请求 html
  // html_get(tar, html_name)await;

  // let html = fs::read_to_string(html_name).expect("file read error");

  // 转化数据结构
  // document_parse(&html, json_name);

  let json = fs::read_to_string(json_name).expect("file read error");
  let v: Value = serde_json::from_str(&json)?;
  
  // 解析
  analyze::page::analyze(&v, &mut link);

  // 数据处理
  dispose_focus(&link, &WholeConfig{
    domain: tar.to_string()
  });

  // println!("{:?}", &link);

  // let json = json!(link);

  // let mut f = fs::File::create(format!("{:?}-2.json", tar)).expect("file create error");
  // f.write_all(json.to_string().as_bytes()).expect("msg");

  Ok(())
}

pub async fn msgot () -> Result<(), Box<dyn std::error::Error>> {

  let json = fs::read_to_string("config.json").unwrap();
  let config: Config = serde_json::from_str(&json)?;

  for va in config.tar.iter() {
    get_analyze_by_tar(va)? 
  }

  Ok(())
}

