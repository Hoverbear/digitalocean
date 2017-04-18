extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;
extern crate uuid;

use std::net::IpAddr;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use digitalocean::request::Retrievable;

use digitalocean::api::Domains;
use digitalocean::DigitalOcean;

use digitalocean::request::Request;
use digitalocean::values::{Domain, DomainRecord};
use digitalocean::action::{Get, List, Create, Update};

// These are more "does it compile" tests.
#[test]
fn usage() {
    let mut request = Domains::get("blah.com")
        .records()
        .create("A", "blah", "192.168.0.1")
        .priority(Some(10));
    
    let mut request = Domains::get("blah.com")
        .records()
        .create("A", "blah", "192.168.0.1");
    request.priority(Some(10));
}

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
    let response = Domains::create(name.clone(), ip_address)
        .retrieve(&digital_ocean);
    println!("Create Domain: {:#?}", response);
    match response {
        Ok(response) => assert_eq!(response.name, name),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Get specific
    let response = Domains::get(name.clone())
        .retrieve(&digital_ocean);
    println!("Get Domain: {:#?}", response);
    match response {
        Ok(response) => assert_eq!(response.name, name),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Check the records
    let response = Domains::get(name.clone())
        .records()
        .retrieve(&digital_ocean);
    println!("List Records: {:#?}", response);
    match response {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Retrieve a record id.
    let record_id = response.unwrap()
        .get(0).unwrap()
        .id;
    
    // Check specific record
    let response = Domains::get(name.clone())
        .records()
        .get(record_id)
        .retrieve(&digital_ocean);
    println!("Get Record: {:#?}", response);
    match response {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Create a specific record
    let response = Domains::get(name.clone())
        .records()
        .create("A", "test", "192.168.0.1")
        .priority(Some(10))
        .ttl(5)
        .retrieve(&digital_ocean);
    println!("Create record {:#?}", response);
    match response {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // Update a specific record.
    let response = Domains::get(name.clone())
        .records()
        .update(record_id)
        .kind("A".into())
        .name("tested".into())
        .data("192.168.0.1".into())
        .ttl(5)
        .retrieve(&digital_ocean);
    println!("Update record {:#?}", response);
    match response {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // Get Domain list
    let response = Domains::list()
        .retrieve(&digital_ocean);
    println!("List Domains: {:#?}", response);
    match response {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    // Delete
    let response = Domains::delete(name.clone())
        .retrieve(&digital_ocean);
    println!("Delete Domain: {:#?}", response);
    match response {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };
}
