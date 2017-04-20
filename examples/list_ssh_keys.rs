extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::SshKey;
use digitalocean::request::Executable;

// cargo run --example list_ssh_keys
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    let req = SshKey::list();
    let result = req.execute(&client)
        .unwrap();

    println!("{:#?}", result);
}