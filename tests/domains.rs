extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;
extern crate uuid;

use digitalocean::{DigitalOcean, Error};
use std::net::IpAddr;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

#[test]
fn endpoints() {
    // Setup for tests
    dotenv().ok();
    env_logger::init()
        .unwrap();
    let api_key = env::var("API_KEY").expect("A valid Digital Ocean API key must be set as API_KEY in .env");

    // Initialization
    let digital_ocean = DigitalOcean::new(api_key)
        .unwrap();
    let domain_api = digital_ocean.domains();

    // Test values
    let name = format!("{}.com", Uuid::new_v4()); // Needs to be unique.
    let ip_address = IpAddr::from_str("1.2.3.4").unwrap();

    // Create
    let domain = domain_api.create(&name, ip_address)
        .unwrap();
    assert!(domain.name == name);

    // Get
    let domain = domain_api.get(&name)
        .unwrap()  // Result
        .unwrap(); // Option
    assert!(domain.name == name);
    domain.do_subresource_thing(false);

    domain_api.get(&name)
        .unwrap() // Result
        .unwrap() // Option
        .do_subresource_thing(false);
    

    // List
    let domains = domain_api.list()
        .unwrap();
    assert!(domains.len() != 0);

    // Delete
    let domain = domain_api.delete(&name)
        .unwrap();
}

// let digital_ocean = DigitalOcean::new(token);
// let action = Droplet::create("foo", "Bar", 50)
//     .backups(true)
//     .ipv6(true)
//     .private_networking(true)
// digital_ocean.execute(action)
//     .unwrap()

// let action = Domain::get("foobar.com")
// digital_ocean.execute(action)
//     .unwrap()