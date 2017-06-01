// Demonstrates the simplest usage.

extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::prelude::*;

// cargo run --example account
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    let req = Account::get();

    let result = client.execute(req).unwrap();

    println!("{:#?}", result);
}
