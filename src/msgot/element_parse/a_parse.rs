use scraper::{ Html, selector::Selector };
use regex::Regex;
pub fn a_parse (document: Html) {
    for node in document.select(&Selector::parse(&"a").unwrap()) {
        let rex = Regex::new(r"^[^\\<>]*$").expect("regex create error");
        if rex.is_match(&node.inner_html()) {
            println!("href: {:?}; content: {:?}", (node.value().attr("href") as Option<&str>).expect(""), node.inner_html());
        }
    }
}
