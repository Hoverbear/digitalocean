// Demonstrates using the API as part of a map chain.

extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Domain;
use digitalocean::Executable;

// cargo run --example records -- $DOMAIN $DOMAIN2...
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    if env::args().len() <= 1 {
        panic!("No domains provided.");
    }

    let results = env::args()
        .skip(1)
        .map(|arg| (arg.clone(), Domain::get(arg).records()))
        .map(|(arg, req)| (arg, req.execute(&client).unwrap()));

    for (arg, records) in results {
        println!("Records for {}:", arg);
        for record in records {
            println!("    {:?}", record)
        }
    }
}
