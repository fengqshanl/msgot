use std::fs::{File};
use std::io::Write;
use std::time::Instant;
use html_parser::Dom;
use html5ever::{driver::{parse_document, ParseOpts}, tree_builder::TreeSink};

pub fn document_parse(html: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>>  {
  let start_time = Instant::now();
  
  let json = Dom::parse(html)?.to_json_pretty()?;

  let mut f = File::create(file_name).expect("file create error");
  f.write_all(json.as_ref()).expect("msg");

  let parse_end_time = Instant::now();
  let parse_elapsed_time = parse_end_time - start_time;
  println!("document parse done");
  println!("document parse运行时间: {} 秒", parse_elapsed_time.as_secs());
  Ok(())
}

pub fn document_parse_in_feature<T>(html: T) where T: TreeSink {
  let _parsed_html = parse_document(html, ParseOpts::default());
  // println!("html: {}", parsedHtml);
}