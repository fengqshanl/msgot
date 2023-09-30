pub mod a_parse;
pub mod img_parse;

use std::fs::File;
use std::io::Write;
use scraper::Html;

pub fn html_parse(html: String) {
  let document: Html = Html::parse_document(&html);

  a_parse::a_parse(document.clone());
  img_parse::img_parse(document.clone());

  let mut a_file = File::create("a_file.text").expect("a file create error");
  a_file.write_all(html.as_ref()).expect("TODO: panic message");
}