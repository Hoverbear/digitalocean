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
    let mut request = Domains::create(name.clone(), ip_address);

    let response = digital_ocean.execute(&request);
    match response {
        Ok(response) => assert_eq!(response.name, name),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Can we run it again? This should fail.
    let response = digital_ocean.execute(&request);
    match response {
        Ok(_) => panic!("Repeated creation of a domain should fail."),
        Err(Error::UnprocessableEntity) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    let request = request.set_zone(String::from("blah"));
    // ... Execute.
}
