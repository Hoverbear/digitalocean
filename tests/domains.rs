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

use digitalocean::api::Domains;
use digitalocean::Retrievable;

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

    // Test values
    let name = format!("{}.com", Uuid::new_v4()); // Needs to be unique.
    let ip_address = IpAddr::from_str("1.2.3.4").unwrap();

    // Create
    let mut response = Domains::create(name.clone(), ip_address)
        .retrieve(&digital_ocean);
    match response {
        Ok(response) => assert_eq!(response.name, name),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Get specific.
    let mut response = Domains::get(name.clone())
        .retrieve(&digital_ocean);
    match response {
        Ok(response) => assert_eq!(response.name, name),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Get list
    let response = Domains::list()
        .retrieve(&digital_ocean);
    match response {
        Ok(response) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Get specific.
    let mut response = Domains::delete(name.clone())
        .retrieve(&digital_ocean);
    match response {
        Ok(response) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };
}
