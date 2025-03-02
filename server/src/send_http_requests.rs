use std::collections::HashMap;

use reqwest::{Error};

use crate::RequestBody;

pub fn send_get_request(url: &str, body: RequestBody) -> Result<String, Error> {
    print!("Hallllooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");

    let client = reqwest::blocking::Client::new();
    println!("body: {:?}", body);

    let response = client.post(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0")
        .header("Connection", "keep-alive")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;
    
    response.text()
}