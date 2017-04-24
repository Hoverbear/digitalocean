extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use std::env;
use digitalocean::DigitalOcean;
use digitalocean::api::Droplet;
use digitalocean::request::Executable;

// cargo run --example droplet -- [id]
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    let mut args = env::args().skip(1);
    let id = args.next();
    let action = args.next();

    match (id, action) {
        (Some(id), Some(action)) => {
            let parsed_id = id.parse::<usize>()
                .expect("Did not pass a valid id.");
            do_droplet_action(&client, parsed_id, action)
        }
        (Some(id), None) => {
            let parsed_id = id.parse::<usize>()
                .expect("Did not pass a valid id.");
            show_droplet_info(&client, parsed_id)
        },
        _ => list_droplets(&client),
    }
}

fn do_droplet_action<S>(client: &DigitalOcean, id: usize, action: S) 
where S: AsRef<str> {
    let req = Droplet::get(id);

    let req = match action.as_ref() {
        "reboot" => req.reboot(),
        "poweroff" => req.power(false),
        "poweron" => req.power(true),
        _ => panic!("Unknown command"),
    };

    let result = req.execute(&client)
        .unwrap();

    println!("{:#?}", result);
}

fn show_droplet_info(client: &DigitalOcean, id: usize) {
    let req = Droplet::get(id);

    let result = req.execute(&client)
        .unwrap();

    println!("{:#?}", result);
}

fn list_droplets(client: &DigitalOcean) {
    let req = Droplet::list();

    let results = req.execute(&client)
        .unwrap();

    for result in results {
        println!("{:#?}", result)
    }
}