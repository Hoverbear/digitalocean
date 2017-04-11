extern crate digitalocean;

use digitalocean::{DigitalOcean, Error};

#[test]
fn list() {
    let digital_ocean = DigitalOcean::new("936af734d6fdcdf8bea41404e29f70a9b372455d94c46cd555ff14fed5692717")
        .unwrap();
    let images = digital_ocean.images().list()
        .unwrap();
    assert!(images.len() != 0);
}

#[test]
fn list_fails_gracefully() {
    let digital_ocean = DigitalOcean::new("invalid")
        .unwrap();
    let result = digital_ocean.images().list();
    match result {
        Err(Error::Unauthorized) => (),
        _ => panic!("Expected request to fail."),
    }
}