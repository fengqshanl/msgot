use std::{collections::HashMap};
use reqwest::Response;

pub async fn request(method: reqwest::Method, url: String, body: String) -> Result<Response, ()>
{
    let mut builder = reqwest::Client::new()
        .request(method.clone(), url)
        .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8");
    match method {
      reqwest::Method::POST | reqwest::Method::PUT => {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("code", &body);
        builder = builder.form(&map);
      },
      _ => {}
    }
    let response = builder.send().await;
    if let Ok(data) = response {
        if data.status().is_success() {
            println!("Deserialize:{:?}", data);
            Ok(data)
        } else {
            println!("error: {:?}", data.status().as_u16());
            Err(())
        }
    } else {
        println!("RequestError");
        Err(())
    }
}
