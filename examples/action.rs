extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Action;
use digitalocean::request::Executable;

// cargo run --example account -- [id]
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    let maybe_id = env::args().nth(2)
        .map(|v| v.parse::<usize>().expect("ID was not integer"));

    match maybe_id {
        Some(id) => {
            let val = Action::get(id)
                .execute(&client)
                .unwrap();
            println!("{:#?}", val);
        },
        None => {
            let val = Action::list()
                .execute(&client)
                .unwrap();
            println!("{:#?}", val);
        },
    }
}