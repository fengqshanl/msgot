use scraper::{ Html, selector::Selector };
pub fn img_parse (document: Html) {
    for node in document.select(&Selector::parse(&"img").unwrap()) {
        match node.value().attr("data-src") {
            Some(src) => {
                println!("img data-src: {:?}", src);
            },
            _ => {
                match node.value().attr("src") {
                    Some(src) => {
                        println!("img src: {:?}", src);
                    },
                    _ => {
                        match node.value().attr("alt") {
                            Some(src) => {
                                println!("img alt: {:?}", src);
                            },
                            _ => {
                                println!("no link");
                            }
                        }
                    }
                }
            }
        }
    }
}
