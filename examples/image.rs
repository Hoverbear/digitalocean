// Demonstrates doing actions on `Request<_,_>`s

extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Image;
use digitalocean::request::Executable;

// cargo run --example image
// cargo run --example image -- $IMAGE
// cargo run --example image -- $IMAGE --actions
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let mut args = env::args().skip(1);

    let id = args.next();

    // Okay this is not ~actually~ checking for `--actions`,
    // but this is an example.
    let actions_flag = args.next().is_some();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    match (id, actions_flag) {
        (Some(id), true) => println!("{:#?}", Image::get(id).actions().execute(&client)),
        (Some(id), false) => println!("{:#?}", Image::get(id).execute(&client)),
        _ => println!("{:#?}", Image::list().execute(&client)),
    }
}
