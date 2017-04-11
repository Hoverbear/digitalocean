extern crate digitalocean;

use digitalocean::{DigitalOcean, Error};

#[test]
fn can_pass_different_api_bits_to_other_threads() {
    let client = DigitalOcean::new("936af734d6fdcdf8bea41404e29f70a9b372455d94c46cd555ff14fed5692717")
        .unwrap();

    let cloned_client = client.clone();
    std::thread::spawn(move || {
        let result = cloned_client.domains().list()
            .unwrap();
    }).join().unwrap();

    let result = client.domains().list();
}