extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Image;
use digitalocean::request::Executable;

/// cargo run --example get_image -- $IMAGE
/// cargo run --example get_image -- $IMAGE --actions
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let mut args = env::args().skip(1);
    
    let id = args.next()
        .expect("No ID specified");

    // Okay this is not ~actually~ checking for `--actions`,
    // but this is an example.
    let actions_flag = args.next().is_some();

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    let req = Image::get(id);

    // Calling `.action()` makes returns a different type than what `req` is.
    match actions_flag {
        true => println!("{:#?}", req.actions().execute(&client)),
        false => println!("{:#?}", req.execute(&client))
    }
}