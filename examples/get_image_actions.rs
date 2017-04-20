extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Image;
use digitalocean::request::Executable;

/// cargo run --example get_image_actions $SOME_IMAGE
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let arg = env::args()
        .skip(1)
        .next()
        .expect("No ID specified");

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    let req = Image::get(arg).actions();
    let result = req.execute(&client)
        .unwrap();

    println!("{:#?}", result);
}