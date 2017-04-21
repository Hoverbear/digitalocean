extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Droplet;
use digitalocean::request::Executable;

// cargo run --example droplet
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    let mut args = env::args().skip(1);
    let id = args.next();

    match id {
        Some(id) => {
            let parsed_id = id.parse::<usize>()
                .expect("Did not pass a valid id.");
            show_droplet_info(&client, parsed_id)
        },
        None => list_droplets(&client),
    }
}

fn show_droplet_info(client: &DigitalOcean, id: usize) {
    let req = Droplet::get(id);

    let result = req.execute(&client)
        .unwrap();

    print_result(result);
}

fn list_droplets(client: &DigitalOcean) {
    let req = Droplet::list();

    let results = req.execute(&client)
        .unwrap();

    for result in results {
        print_result(result)
    }
}

fn print_result(result: Droplet) {
    println!("id: {}, name: {}, size: {}, status: {}", 
        result.id,
        result.name,
        result.size.slug,
        result.status);
}