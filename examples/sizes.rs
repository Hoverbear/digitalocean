// Demonstrates `DigitalOcean::execute(...)`

extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::prelude::*;

// cargo run --example sizes
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    let result = client.execute(Size::list()).unwrap();

    println!("{:#?}", result);
}
