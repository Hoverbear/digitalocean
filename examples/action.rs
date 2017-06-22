// Demonstrates using `Request<_,_>::execute()`

extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use digitalocean::prelude::*;
use std::env;

enum Choice {
    List(Option<usize>),
    Get(usize),
}

// cargo run --example action -- [--list [limit] | --id id]
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    let mut args = env::args().skip(1);
    let choice = args.next().expect("No action specified.");
    let param = args.next().map(|v| {
        v.parse::<usize>().expect("Param was not integer")
    });

    let choice = match choice.as_ref() {
        "--list" => Choice::List(param),
        "--id" => Choice::Get(param.expect("No ID specified")),
        _ => panic!("Invalid args"),
    };

    match choice {
        Choice::Get(id) => {
            let val = Action::get(id).execute(&client).unwrap();
            println!("{:#?}", val);
        },
        Choice::List(limit) => {
            let val = Action::list().limit(limit).execute(&client).unwrap();
            println!("{:#?}", val);
            println!("Total actions: {:#?}", val.len());
        },
    }
}
