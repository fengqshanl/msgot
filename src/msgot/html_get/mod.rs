use std::fs::{File};
use std::io::Write;
use crate::client::reqwest::request;
use scraper::{ Html };

pub async fn html_get(url: &str, file_name: &str) {
  let res = request(reqwest::Method::GET, url.to_owned(), "".to_string()).await.unwrap();
  let html = res.text().await.unwrap();
  // let document = Html::parse_document(&html);
  let mut f = File::create(format!("{}.html", file_name)).expect("file create err");
  f.write_all(html.as_ref()).expect("html write error");
}